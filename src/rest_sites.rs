use crate::*;
use bevy_mod_picking::events::{Click, Out, Over, Pointer};
use bevy_mod_picking::prelude::On;

pub struct RestSitePlugin;

impl Plugin for RestSitePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::RestSite), spawn_stuff)
            .add_event::<OpenCardSelectionMenuEvent>()
            .add_systems(
                Update,
                (
                    open_card_selection_menu_waiter,
                    button_system,
                    make_smithing_result,
                )
                    .distributive_run_if(in_state(GameState::RestSite)),
            )
            .add_systems(OnExit(GameState::RestSite), clean_up);
    }
}

#[derive(Component)]
struct RestSiteThing;

#[derive(Component, Debug, Reflect)]
pub struct SelectedCard(Card);

#[derive(Event)]
pub enum OpenCardSelectionMenuEvent {
    OpenMenu(Vec<Card>, Vec<SelectedCardAction>),
    CardSelected((Card, Vec<SelectedCardAction>)),
}

#[derive(Clone)]
pub enum SelectedCardAction {
    RemoveFromDeck,
    ReturnSelectedCardInfo(Transform),
    AddToDeck(Option<Vec<Card>>),
}

fn spawn_card_selector(commands: &mut Commands, actions: Vec<SelectedCardAction>, pos: Transform) {
    let mut binding: bevy::ecs::system::EntityCommands = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::DARK_GRAY,
            custom_size: Some(Vec2 { x: 125., y: 200. }),
            ..default()
        },
        transform: pos,
        ..default()
    });
    let card_1_selector = binding
        .insert(On::<Pointer<Over>>::target_component_mut::<Sprite>(
            |_, sprite| {
                sprite.color.set_l(1.);
            },
        ))
        .insert(On::<Pointer<Out>>::target_component_mut::<Sprite>(
            |_, sprite| {
                sprite.color.set_l(0.5);
            },
        ))
        .insert(RestSiteThing)
        .insert(Name::new("Button for spawning the stuff"));
    let card_1_entity = card_1_selector.id();
    card_1_selector.insert(On::<Pointer<Click>>::run(
        move |mut open_card_selection_menu_writer: EventWriter<OpenCardSelectionMenuEvent>,
              player: Res<Player>,
              mut commands: Commands| {
            open_card_selection_menu_writer.send(OpenCardSelectionMenuEvent::OpenMenu(
                player.deck.clone(),
                actions.clone(),
            ));
            commands.entity(card_1_entity).despawn_recursive();
        },
    ));
}

fn spawn_stuff(mut commands: Commands) {
    let actions = vec![
        SelectedCardAction::RemoveFromDeck,
        SelectedCardAction::ReturnSelectedCardInfo(Transform::from_xyz(-175., 150., 0.)),
    ];
    spawn_card_selector(&mut commands, actions, Transform::from_xyz(-175., 150., 0.));
    let actions = vec![
        SelectedCardAction::RemoveFromDeck,
        SelectedCardAction::ReturnSelectedCardInfo(Transform::from_xyz(175., 150., 0.)),
    ];
    spawn_card_selector(&mut commands, actions, Transform::from_xyz(175., 150., 0.));
}

#[derive(Component)]
struct CardSelectMenuItem;
#[derive(Component)]
struct Actions {
    pub actions: Vec<SelectedCardAction>,
}

fn open_card_selection_menu_waiter(
    mut open_card_selection_menu_event_reader: EventReader<OpenCardSelectionMenuEvent>,
    mut player: ResMut<Player>,
    mut commands: Commands,
    menu_items: Query<Entity, With<CardSelectMenuItem>>,
) {
    for event in open_card_selection_menu_event_reader.read() {
        match event {
            OpenCardSelectionMenuEvent::OpenMenu(cards, card_actions) => {
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
                    .insert(CardSelectMenuItem)
                    .insert(Actions {
                        actions: card_actions.clone(),
                    })
                    .with_children(|parent| {
                        for card in cards {
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
                                .insert(CombatThing)
                                .insert(card.clone())
                                .insert(CustomButton::new(
                                    format!("{}", card.description).to_string(),
                                    format!("{}", card.description).to_string(),
                                    "Pick a Target".to_string(),
                                    true,
                                ))
                                .insert(Name::new("Card"))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            "",
                                            TextStyle {
                                                font_size: 20.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                ..default()
                                            },
                                        )
                                        .with_alignment(TextAlignment::Center),
                                        style: Style {
                                            align_self: AlignSelf::End,
                                            height: Val::Percent(70.),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                        }
                    });
            }
            OpenCardSelectionMenuEvent::CardSelected((card, actions)) => {
                for item in menu_items.iter() {
                    commands.entity(item).despawn_recursive();
                }
                for action in actions {
                    match action {
                        SelectedCardAction::AddToDeck(opt_card) => {
                            if let Some(cards) = opt_card {
                                for card in cards {
                                    player.deck.push(card.clone())
                                }
                            } else {
                                player.deck.push(card.clone())
                            }
                        }
                        SelectedCardAction::RemoveFromDeck => {
                            for i in 0..player.deck.len() {
                                if player.deck[i] == *card {
                                    player.deck.remove(i);
                                    break;
                                }
                            }
                        }
                        SelectedCardAction::ReturnSelectedCardInfo(pos) => {
                            commands
                                .spawn(SpriteBundle {
                                    sprite: Sprite {
                                        color: Color::Rgba {
                                            red: 1.0,
                                            green: 0.5,
                                            blue: 0.9,
                                            alpha: 1.0,
                                        },
                                        custom_size: Some(Vec2 { x: 125., y: 200. }),
                                        ..default()
                                    },
                                    transform: *pos,
                                    ..default()
                                })
                                .insert(Name::new("Card"))
                                .insert(SelectedCard(card.clone()))
                                .with_children(|parent| {
                                    parent.spawn(Text2dBundle {
                                        text: Text::from_section(
                                            card.description.clone(),
                                            TextStyle {
                                                font_size: 20.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                ..default()
                                            },
                                        ),
                                        transform: Transform {
                                            translation: Vec3::new(0.0, -35.0, 1.0),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                        }
                    }
                }
            }
        }
    }
}

fn make_smithing_result(mut commands: Commands, cards: Query<(&SelectedCard, Entity)>) {
    if cards.iter().len() == 2 {
        let mut effects: Vec<Effect> = vec![];
        let mut cost_total = 0;
        for (selected_card, entity) in &cards {
            for effect in &selected_card.0.effects {
                effects.push(effect.clone())
            }
            cost_total += selected_card.0.cost;
            commands.entity(entity).despawn_recursive();
        }
        let card = Card::new(effects, cost_total);
        let mut binding: bevy::ecs::system::EntityCommands = commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                custom_size: Some(Vec2 { x: 125., y: 200. }),
                ..default()
            },
            transform: Transform::from_xyz(0., -200., 0.),
            ..default()
        });
        let card_1_selector = binding
            .insert(On::<Pointer<Over>>::target_component_mut::<Sprite>(
                |_, sprite| {
                    sprite.color.set_l(1.);
                },
            ))
            .insert(On::<Pointer<Out>>::target_component_mut::<Sprite>(
                |_, sprite| {
                    sprite.color.set_l(0.5);
                },
            ))
            .insert(RestSiteThing)
            .insert(Name::new("Button for spawning the stuff"));
        let card_1_entity = card_1_selector.id();
        card_1_selector.insert(On::<Pointer<Click>>::run(
            move |mut player: ResMut<Player>, mut commands: Commands| {
                player.deck.push(card.clone());
                commands.entity(card_1_entity).despawn_recursive();
            },
        ));
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &CustomButton,
            &Card,
            Entity,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut selected_card_writer: EventWriter<OpenCardSelectionMenuEvent>,
    node_q: Query<&Actions, With<Node>>,
) {
    for (interaction, mut color, mut border_color, children, button_data, card, _entity) in
        &mut interaction_query
    {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = button_data.pressed_text.clone();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                selected_card_writer.send(OpenCardSelectionMenuEvent::CardSelected((
                    card.clone(),
                    node_q.single().actions.clone(),
                )))
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

fn clean_up(mut commands: Commands, things: Query<Entity, With<RestSiteThing>>) {
    for thing in &things {
        commands.entity(thing).despawn_recursive()
    }
}
