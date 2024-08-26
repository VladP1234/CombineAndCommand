use crate::*;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (display_deck, remove_tooltip).run_if(resource_exists::<Player>()),
        )
        .add_systems(Update, tooltip.run_if(resource_added::<Player>()));
    }
}

#[derive(Component)]
struct DeckDisplayItem;

fn display_deck(
    player: Res<Player>,
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    deck_display_items: Query<Entity, With<DeckDisplayItem>>,
) {
    if keyboard_input.just_pressed(KeyCode::D) {
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Grid,
                    grid_template_columns: vec![RepeatedGridTrack::percent(10, 10.)],
                    grid_template_rows: vec![RepeatedGridTrack::percent(4, 25.)],
                    ..default()
                },
                background_color: Color::Rgba {
                    red: 0.1,
                    green: 0.1,
                    blue: 0.1,
                    alpha: 0.5,
                }
                .into(),
                ..default()
            })
            .insert(DeckDisplayItem)
            .with_children(|parent| {
                for card in &player.deck {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Percent(90.),
                                height: Val::Percent(90.),
                                border: UiRect::all(Val::Px(5.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                align_self: AlignSelf::FlexEnd,
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .insert(CustomButton::new(
                            format!("{}", card.description).to_string(),
                            format!("{}", card.description).to_string(),
                            format!("{}", card.description).to_string(),
                            true,
                        ))
                        .insert(Name::new("Card"))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    format!("{}", card.description).to_string(),
                                    TextStyle {
                                        font_size: 20.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                        ..default()
                                    },
                                )
                                .with_alignment(TextAlignment::Center),
                                style: Style {
                                    // align_self: AlignSelf::End,
                                    // height: Val::Percent(70.),
                                    ..default()
                                },
                                ..default()
                            });
                        });
                }
            });
    } else if keyboard_input.just_released(KeyCode::D) {
        for entity in &deck_display_items {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component)]
struct Tooltip;
fn tooltip(mut commands: Commands) {
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "Press 'D' to view your deck",
                TextStyle {
                    font_size: 20.0,
                    color: Color::rgba(0.9, 0.9, 0.9, 0.5),
                    ..default()
                },
            ),
            ..default()
        })
        .insert(Tooltip);
}

fn remove_tooltip(
    mut commands: Commands,
    tooltips: Query<Entity, With<Tooltip>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::D) {
        for tooltip in tooltips.iter() {
            commands.entity(tooltip).despawn_recursive()
        }
    }
}
