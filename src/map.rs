use crate::*;
use bevy::input::mouse::MouseWheel;
use bevy_mod_picking::events::Out;
use bevy_mod_picking::events::{Click, Over, Pointer};
use bevy_mod_picking::prelude::On;
use bevy_mod_picking::PickableBundle;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::f32;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Map), generate_map)
            .add_systems(
                Update,
                (scroll).distributive_run_if(in_state(GameState::Map)),
            )
            .add_systems(OnExit(GameState::Map), clean_up);
    }
}

#[derive(Resource, Default, Reflect)]
pub struct MapManager {
    map_data: Option<HashMap<(i32, i32), Vec<(i32, i32)>>>,
    current_tile: (i32, i32),
}

#[derive(Component)]
struct MapThing;

fn clean_up(mut commands: Commands, map_things: Query<Entity, With<MapThing>>) {
    for map_thing in &map_things {
        commands.entity(map_thing).despawn_recursive()
    }
}

// Converted proof of concept Python Script to Rust with GPT 4 (generate_map_data + add_connections)
fn generate_map_data() -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    let mut connections: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    let mut rng = thread_rng();

    for _ in 0..3 {
        let mut current_tile = (rng.gen_range(0..=4), 0);
        for _ in 0..(START_TILE.1 - 1) {
            let new_tile = (
                rng.gen_range((current_tile.0 - 1).max(0)..=(current_tile.0 + 1).min(4)),
                current_tile.1 + 1,
            );
            add_connection(&mut connections, current_tile, new_tile);
            current_tile = new_tile;
        }
        add_connection(&mut connections, current_tile, START_TILE)
    }

    connections
}

fn add_connection(
    connections: &mut HashMap<(i32, i32), Vec<(i32, i32)>>,
    from_tile: (i32, i32),
    to_tile: (i32, i32),
) {
    connections
        .entry(from_tile)
        .or_insert_with(Vec::new)
        .push(to_tile);
}

fn generate_map(mut commands: Commands, opt_map_manager: Option<ResMut<MapManager>>) {
    let mut generated_tiles: Vec<(i32, i32)> = Vec::new();
    let map_data: HashMap<(i32, i32), Vec<(i32, i32)>>;
    let mut current_tile: (i32, i32) = START_TILE;
    if let Some(mut map_manager) = opt_map_manager {
        if let Some(data) = &map_manager.map_data {
            map_data = data.clone();
            current_tile = map_manager.current_tile;
        } else {
            map_data = generate_map_data();
            map_manager.map_data = Some(map_data.clone())
        }
    } else {
        map_data = generate_map_data();
        commands.insert_resource(MapManager {
            map_data: Some(map_data.clone()),
            current_tile: START_TILE,
        })
    }

    for (end_tile, start_tiles) in map_data {
        let x1 = end_tile.0 as f32 * 230. - 460.;
        let y1 = (end_tile.1 - current_tile.1 + 2) as f32 * 300. - 300.;
        for start_tile in &start_tiles {
            let x2 = start_tile.0 as f32 * 230.0 - 460.;
            let y2 = (start_tile.1 - current_tile.1 + 2) as f32 * 300. - 300.;

            let height = ((x1 - x2).powf(2.) as f32 + (y1 - y2 + 100.).powf(2.) as f32).sqrt();
            let angle = (-(x2 - x1) / (y2 - y1 - 100.)).atan();
            // Connecting Rod
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GRAY,
                        custom_size: Some(Vec2::new(5.0, height)),
                        ..default()
                    },
                    transform: Transform::from_xyz((x1 + x2) / 2.0, (y1 + y2) / 2.0, 0.)
                        .with_rotation(Quat::from_rotation_z(angle)),
                    ..Default::default()
                })
                .insert(MapThing)
                .insert(Name::new("Connecting Rod"));
            if !generated_tiles.contains(&end_tile) {
                spawn_tile(
                    &mut commands,
                    (x1, y1),
                    &current_tile,
                    &start_tiles,
                    &end_tile,
                );
                generated_tiles.push(end_tile);
            }
        }
    }
}

fn spawn_tile(
    commands: &mut Commands,
    pos: (f32, f32),
    current_tile: &(i32, i32),
    start_tiles: &Vec<(i32, i32)>,
    spawning_tile: &(i32, i32),
) {
    let bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::DARK_GRAY,
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        transform: Transform::from_xyz(pos.0, pos.1, 0.),
        ..Default::default()
    };
    let mut binding = commands.spawn(bundle);
    let spawn = &mut binding.insert(Name::new("Encounter"));
    let tile = spawn.insert(MapThing);
    let c_tile = spawning_tile.clone();
    for start_tile in start_tiles {
        if current_tile == start_tile {
            tile.insert(Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            });
            tile.insert(PickableBundle::default());
            tile.insert(On::<Pointer<Click>>::run(
                move |mut next_state: ResMut<NextState<GameState>>,
                      mut map_manager: ResMut<MapManager>| {
                    next_state.set(GameState::Combat);
                    map_manager.current_tile = c_tile
                },
            ));
            tile.insert(On::<Pointer<Over>>::target_component_mut::<Sprite>(
                |_, sprite| {
                    sprite.color.set_l(1.);
                },
            ));
            tile.insert(On::<Pointer<Out>>::target_component_mut::<Sprite>(
                |_, sprite| {
                    // debug!("{}", sprite.color.l())
                    sprite.color.set_l(0.5);
                },
            ));
        }
    }
}

fn scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<MapThing>>,
) {
    for event in mouse_wheel_events.read() {
        for mut transform in query.iter_mut() {
            // Adjust this value to control the zoom speed
            let scroll_speed = -0.3;
            let scroll_amount = 1.0 + event.y * scroll_speed;

            // Scale the camera transform to zoom
            transform.translation += Vec3::new(0.0, scroll_amount, 0.0);
            // if transform.translation.y < -150.0 {
            //     transform.translation.y = -150.0
            // } else if transform.translation.y > 3300.0 {
            //     transform.translation.y = 3300.0
            // }
        }
    }
}
