use crate::*;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_state::<GameState>()
            .add_event::<BaseEvent>()
            .add_systems(
                Update,
                (button_system, base_event_manager).run_if(in_state(GameState::HomeBase)),
            );
    }
}

#[derive(Component)]
struct BaseThing;

#[derive(Event, Component, Clone)]
pub enum BaseEvent {
    LoadHomeScreen,
    LoadLeaderSelectionMenu,
    StartGame(Vec<Card>, Vec<(Unit, Attack)>),
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &CustomButton,
            &BaseEvent,
            Entity,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut base_event_writer: EventWriter<BaseEvent>,
) {
    for (interaction, mut color, mut border_color, children, button_data, base_event, _entity) in
        &mut interaction_query
    {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = button_data.pressed_text.clone();
                *color = PRESSED_BUTTON.into();
                base_event_writer.send(base_event.clone())
            }
            Interaction::Hovered => {
                text.sections[0].value = button_data.hover_text.clone();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = button_data.off_text.clone();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

// The things on the home screen are created by this system
fn base_event_manager(
    mut base_event_reader: EventReader<BaseEvent>,
    mut commands: Commands,
    things: Query<Entity, With<BaseThing>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in base_event_reader.read() {
        for thing in &things {
            commands.entity(thing).despawn_recursive();
        }
        match event {
            // Makes the start game button, event occurs on startup or when the player is killed.
            BaseEvent::LoadHomeScreen => {
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(BaseThing)
                    .with_children(|parent| {
                        parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(400.0),
                                    height: Val::Px(130.0),
                                    border: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .insert(CustomButton::new(
                                "Start Game".to_string(),
                                "Start Game".to_string(),
                                "Start Game".to_string(),
                                false,
                            ))
                            .insert(BaseEvent::LoadLeaderSelectionMenu)
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Start Game",
                                    TextStyle {
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                        ..default()
                                    },
                                ));
                            });
                    });
            }
            // Event occurs when the start game button on the start screen is pressed
            BaseEvent::LoadLeaderSelectionMenu => {
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(BaseThing)
                    .with_children(|parent| {
                        // The same deck is used for all leaders since I have no inscentive to make a unique one for each one and this is sufficient to demonstrate how the deck feature works
                        let mut deck: Vec<Card> = Vec::new();
                        deck.push(Card::new(Effect::BonusDamage(-2), 1));
                        deck.push(Card::new(Effect::BonusHealth(-2), 1));
                        deck.push(Card::new(Effect::BonusCountdown(1), 1));
                        deck.push(Card::new(
                            vec![Effect::DealDamage(4), Effect::BonusDamage(6)],
                            1,
                        ));
                        deck.push(Card::new(
                            vec![Effect::DealDamage(3), Effect::BonusHealth(6)],
                            1,
                        ));
                        // Same goes for the units
                        let mut p_units: Vec<(Unit, Attack)> = Vec::new();
                        p_units.push((Unit::new(10, (0, 0)), Attack::new(4, 3)));
                        p_units.push((Unit::new(10, (0, 1)), Attack::new(9, 100)));
                        p_units.push((Unit::new(10, (1, 0)), Attack::new(6, 5)));
                        // Makes the button for selecting the first leader
                        parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Percent(20.),
                                    height: Val::Percent(70.),
                                    margin: UiRect::all(Val::Percent(5.)),
                                    border: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .insert(CustomButton::new(
                                "Leader 1".to_string(),
                                "Leader 1".to_string(),
                                "Leader 1".to_string(),
                                false,
                            ))
                            .insert(BaseEvent::StartGame(deck.clone(), p_units.clone()))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Leader 1",
                                    TextStyle {
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                        ..default()
                                    },
                                ));
                            });
                        // Makes the button for selecting the second leader
                        parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Percent(20.),
                                    height: Val::Percent(70.),
                                    margin: UiRect::all(Val::Percent(5.)),
                                    border: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .insert(CustomButton::new(
                                "Leader 2".to_string(),
                                "Leader 2".to_string(),
                                "Leader 2".to_string(),
                                false,
                            ))
                            .insert(BaseEvent::StartGame(deck.clone(), p_units.clone()))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Leader 2",
                                    TextStyle {
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                        ..default()
                                    },
                                ));
                            });
                        // Makes the button for selecting the third leader
                        parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Percent(20.),
                                    height: Val::Percent(70.),
                                    margin: UiRect::all(Val::Percent(5.)),
                                    border: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .insert(CustomButton::new(
                                "Leader 3".to_string(),
                                "Leader 3".to_string(),
                                "Leader 3".to_string(),
                                false,
                            ))
                            .insert(BaseEvent::StartGame(deck, p_units))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Leader 3",
                                    TextStyle {
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                        ..default()
                                    },
                                ));
                            });
                    });
            }
            // Evvent occurs when a leader is selected
            BaseEvent::StartGame(deck, units) => {
                commands.insert_resource(Player::new(deck.clone(), units.clone()));
                next_state.set(GameState::GenerateMap);
            }
        }
    }
}

// Occurs on startup
fn setup(mut commands: Commands, mut base_event_writer: EventWriter<BaseEvent>) {
    commands.spawn(Camera2dBundle::default());
    base_event_writer.send(BaseEvent::LoadHomeScreen);
}
