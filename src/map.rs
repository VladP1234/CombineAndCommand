use crate::*;
use bevy::input::mouse::MouseWheel;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::f32;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Map), generate_map)
            .add_systems(
                Update,
                (scroll, button_system).distributive_run_if(in_state(GameState::Map)),
            );
    }
}

#[derive(Resource, Default)]
pub struct MapManager {
    map_data: Option<HashMap<(i32, i32), Vec<(i32, i32)>>>,
}

// Converted proof of concept Python Script to Rust with GPT 4 (generate_map_data + add_connections)
fn generate_map_data() -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    let mut connections: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    let mut rng = thread_rng();

    for _ in 0..3 {
        let mut current_tile = (rng.gen_range(0..=4), 0);
        for _ in 0..11 {
            let new_tile = (
                rng.gen_range((current_tile.0 - 1).max(0)..=(current_tile.0 + 1).min(4)),
                current_tile.1 + 1,
            );
            add_connection(&mut connections, current_tile, new_tile);
            current_tile = new_tile;
        }
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
    if let Some(mut map_manager) = opt_map_manager {
        if let Some(data) = &map_manager.map_data {
            map_data = data.clone();
        } else {
            map_data = generate_map_data();
            map_manager.map_data = Some(map_data.clone())
        }
    } else {
        map_data = generate_map_data();
        commands.insert_resource(MapManager {
            map_data: Some(map_data.clone()),
        })
    }

    for (start_tile, end_tiles) in map_data {
        let x1 = start_tile.0 as f32 * 230. - 460.;
        let y1 = start_tile.1 as f32 * 300. - 300.;
        for end_tile in end_tiles {
            let x2 = end_tile.0 as f32 * 230.0 - 460.;
            let y2 = end_tile.1 as f32 * 300. - 300.;

            let height = ((x1 - x2).powf(2.) as f32 + (y1 - y2 + 100.).powf(2.) as f32).sqrt();
            let angle = (-(x2 - x1) / (y2 - y1 - 100.)).atan();
            // Connecting Rod
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2::new(5.0, height)),
                    ..default()
                },
                transform: Transform::from_xyz((x1 + x2) / 2.0, (y1 + y2) / 2.0, 0.)
                    .with_rotation(Quat::from_rotation_z(angle)),
                ..Default::default()
            });
            // Start Tile
            if !generated_tiles.contains(&start_tile) {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(100., 100.)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x1, y1, 0.)
                        .with_rotation(Quat::from_rotation_z(0.)),
                    ..Default::default()
                });
                generated_tiles.push(start_tile);
            }
            // End Tile
            if !generated_tiles.contains(&end_tile) {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(100., 100.)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x2, y2, 0.)
                        .with_rotation(Quat::from_rotation_z(0.)),
                    ..Default::default()
                });
                generated_tiles.push(end_tile);
            }
        }
    }
}

fn scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for event in mouse_wheel_events.read() {
        for mut transform in query.iter_mut() {
            // Adjust this value to control the zoom speed
            let scroll_speed = 0.3;
            let scroll_amount = 1.0 + event.y * scroll_speed;

            // Scale the camera transform to zoom
            transform.translation += Vec3::new(0.0, scroll_amount, 0.0);
            if transform.translation.y < -150.0 {
                transform.translation.y = -150.0
            } else if transform.translation.y > 3300.0 {
                transform.translation.y = 3300.0
            }
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            Entity,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameState>>,
    // mut commands: Commands,
    mut nodes: Query<&mut Visibility, With<Node>>,
) {
    // let mut node_obj = node.single_mut();
    for (interaction, mut color, mut border_color, children, _entity) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                next_state.set(GameState::Combat);
                // commands.entity(entity).despawn_recursive();
                for mut node_obj in &mut nodes {
                    *node_obj = Visibility::Hidden
                }
            }
            Interaction::Hovered => {
                text.sections[0].value = "".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
