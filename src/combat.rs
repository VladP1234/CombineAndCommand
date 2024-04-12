use crate::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Combat), spawn_stuff)
            .add_event::<EndOfTurnEvent>()
            .add_event::<CardEvent>()
            .add_systems(
                Update,
                (
                    update_unit_ui,
                    button_system.before(handle_end_of_turn),
                    handle_end_of_turn,
                    deal_damage_system.after(handle_end_of_turn),
                    provide_player_with_options_system,
                    remove_dead_units.after(deal_damage_system),
                    end_combat,
                )
                    .distributive_run_if(in_state(GameState::Combat)),
            )
            .add_systems(OnExit(GameState::Combat), clean_up);
    }
}

#[derive(Reflect)]
pub enum Turn {
    Player,
    Enemy,
}

// enum CombatState {
//     Fight,
//     Reward,
// }
#[derive(Resource, Reflect)]
pub struct CombatManager {
    turn: Turn,
    remaining_energy: i32,
    max_energy: i32,
    selected_card: Option<(Card, Entity)>,
    deck: Vec<Card>,
}

impl CombatManager {
    fn new(deck: Vec<Card>, energy: i32) -> CombatManager {
        CombatManager {
            turn: Turn::Player,
            remaining_energy: energy,
            max_energy: energy,
            selected_card: None,
            deck,
        }
    }
}
#[derive(Component)]
struct CombatThing;

#[derive(Component, Reflect, Debug)]
pub struct Unit {
    hp: i32,
    pos: (i32, i32),
}

impl Unit {
    fn new(hp: i32, pos: (i32, i32)) -> Unit {
        Unit { hp, pos }
    }
}

#[derive(Component)]
struct IsFriendly;

#[derive(Component, Reflect, Debug)]
pub struct Attack {
    attack_interval: i32,
    remaining_turns: i32,
    damage: i32,
    // TODO: Damage Type
}
impl Attack {
    fn new(attack_interval: i32, damage: i32) -> Attack {
        Attack {
            attack_interval,
            remaining_turns: attack_interval,
            damage,
        }
    }
    fn countdown(&mut self) {
        self.remaining_turns -= 1;
    }
}

#[derive(Component)]
pub enum CombatButtonType {
    EndTurn,
    Card(Card),
    UnitSelector(Entity),
}

#[derive(Component)]
enum UnitTextMarker {
    Countdown,
    Damage,
    Hp,
}

#[derive(Event)]
struct EndOfTurnEvent;

#[derive(Event)]
enum CardEvent {
    CardSelected((Card, Entity)),
    CardDeselected(Option<Entity>),
}

fn spawn_unit(commands: &mut Commands, unit: Unit, is_friendly: bool, attack: Attack) {
    let countdown = attack.attack_interval.to_string();
    let hp = unit.hp.to_string();
    let damage = attack.damage.to_string();
    // lifetime issues so I have to spawn like this
    let mut binding = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: if is_friendly {
                Color::rgba(0.25, 0.25, 0.75, 0.7)
            } else {
                Color::rgba(0.75, 0.25, 0.25, 0.7)
            },
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        ..default()
    });
    let mod_unit = binding
        .insert(unit)
        .insert(attack)
        .insert(CombatThing)
        .with_children(|parent| {
            // Attack Countdown
            parent
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        countdown,
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgba(0.9, 0.9, 0.9, 1.0),
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0., 75., 0.),
                    ..default()
                })
                .insert(UnitTextMarker::Countdown);
            // Hp
            parent
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        hp,
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgba(0.9, 0.9, 0.9, 1.0),
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(-45., -65., 0.),
                    ..default()
                })
                .insert(UnitTextMarker::Hp);
            // Damage
            parent
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        damage,
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgba(0.9, 0.9, 0.9, 1.0),
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(45., -65., 0.),
                    ..default()
                })
                .insert(UnitTextMarker::Damage);
        });

    if is_friendly {
        mod_unit.insert(IsFriendly);
    }
}

fn make_card(commands: &mut Commands, card: Card) -> Entity {
    let mut binding = commands.spawn(ButtonBundle {
        style: Style {
            width: Val::Percent(12.5),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            align_self: AlignSelf::FlexEnd,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    });
    let card = binding
        .insert(CombatThing)
        .insert(CombatButtonType::Card(card))
        .insert(CustomButton::new(
            "".to_string(),
            "".to_string(),
            "Pick a Target".to_string(),
            true,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
    card.id()
}

fn spawn_stuff(mut commands: Commands) {
    let mut deck: Vec<Card> = Vec::new();
    deck.push(Card::new(Target::Enemy, Effect::BonusDamage(-2), 1));
    deck.push(Card::new(Target::Enemy, Effect::BonusHealth(-2), 1));
    deck.push(Card::new(Target::Enemy, Effect::BonusCountdown(1), 1));
    commands.insert_resource(CombatManager::new(deck, 3));
    spawn_unit(
        &mut commands,
        Unit::new(10, (0, 0)),
        true,
        Attack::new(1, 10),
    );
    spawn_unit(
        &mut commands,
        Unit::new(10, (0, 0)),
        false,
        Attack::new(1, 1),
    );
    let card1 = make_card(
        &mut commands,
        Card::new(Target::Enemy, Effect::BonusHealth(-2), 1),
    );
    let card2 = make_card(
        &mut commands,
        Card::new(Target::Player, Effect::BonusDamage(2), 1),
    );
    let card3 = make_card(
        &mut commands,
        Card::new(Target::Player, Effect::BonusCountdown(-1), 1),
    );
    let mut starter_cards: Vec<Entity> = Vec::new();
    starter_cards.push(card1);
    starter_cards.push(card2);
    starter_cards.push(card3);
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(30.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                top: Val::Percent(70.0),
                ..default()
            },
            background_color: Color::rgba(0., 0.1, 0.1, 0.1).into(),
            ..default()
        })
        .insert(CombatThing)
        .insert_children(0, &starter_cards);

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(30.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                top: Val::Percent(70.0),
                right: Val::Px(0.),
                justify_self: JustifySelf::End,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(33.0),
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
                .insert(CombatButtonType::EndTurn)
                .insert(CustomButton::new(
                    "End Turn".to_string(),
                    "Sure?".to_string(),
                    "Ended Turn".to_string(),
                    false,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "End Turn",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &CombatButtonType,
            &mut CustomButton,
            Entity,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut cb_manager: ResMut<CombatManager>,
    mut end_of_turn_event_writer: EventWriter<EndOfTurnEvent>,
    mut card_selection_event_writer: EventWriter<CardEvent>,
    mut units: Query<(&mut Unit, &mut Attack, Entity)>,
) {
    for (
        interaction,
        mut color,
        mut border_color,
        children,
        button_type,
        mut button_data,
        entity,
    ) in &mut interaction_query
    {
        let mut text = text_query.get_mut(children[0]).unwrap();
        if !button_data.enabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = button_data.pressed_text.clone();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::WHITE;
                if let Some(toggle_state) = button_data.toggled {
                    button_data.toggled = Some(!toggle_state)
                }
                match button_type {
                    CombatButtonType::EndTurn => {
                        cb_manager.turn = Turn::Enemy;
                        button_data.enabled = false;
                        *color = NORMAL_BUTTON.into();
                        border_color.0 = Color::BLACK;
                        end_of_turn_event_writer.send(EndOfTurnEvent);
                    }
                    CombatButtonType::Card(card_data) => {
                        if button_data.toggled.unwrap() {
                            if cb_manager.remaining_energy < card_data.cost {
                                button_data.toggled = Some(!button_data.toggled.unwrap());
                                continue;
                            }
                            card_selection_event_writer.send(CardEvent::CardDeselected(None));
                            card_selection_event_writer
                                .send(CardEvent::CardSelected((card_data.clone(), entity)));
                        } else {
                            card_selection_event_writer.send(CardEvent::CardDeselected(None));
                            cb_manager.selected_card = None
                        }
                    }
                    CombatButtonType::UnitSelector(chosen_unit) => {
                        *color = Color::rgb(0.2, 0.8, 0.2).into();
                        for (mut unit, mut attack, entity) in units.iter_mut() {
                            if chosen_unit == &entity {
                                let mut cost = 0;
                                if let Some((ref card, entity)) = cb_manager.selected_card {
                                    cost = card.cost;
                                    match card.effect {
                                        Effect::BonusDamage(damage) => attack.damage += damage,
                                        Effect::BonusHealth(health) => unit.hp += health,
                                        Effect::BonusCountdown(countdown) => {
                                            attack.remaining_turns += countdown
                                        }
                                    }
                                    card_selection_event_writer
                                        .send(CardEvent::CardDeselected(Some(entity)));
                                }
                                cb_manager.remaining_energy -= cost;
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                if let Some(state) = button_data.toggled {
                    if state {
                        continue;
                    }
                }
                text.sections[0].value = button_data.hover_text.clone();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::GRAY;
                if let CombatButtonType::UnitSelector(_) = button_type {
                    *color = Color::rgb(0.2, 0.8, 0.2).into();
                }
            }
            Interaction::None => {
                if let Some(state) = button_data.toggled {
                    if state {
                        continue;
                    }
                }
                text.sections[0].value = button_data.off_text.clone();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
                if let CombatButtonType::UnitSelector(_) = button_type {
                    *color = Color::rgb(0.2, 0.6, 0.2).into();
                }
            }
        }
    }
}

fn update_unit_ui(
    mut units: Query<(
        &mut Transform,
        &Unit,
        &mut Attack,
        Entity,
        Option<&IsFriendly>,
    )>,
    mut texts_query: Query<(&mut Text, &Parent, &UnitTextMarker)>,
) {
    for (mut unit_pos, unit_struct, attack, parent_entity, is_friendly) in units.iter_mut() {
        // Determine position based on whether the unit is friendly or enemy
        unit_pos.translation = if is_friendly.is_some() {
            Vec3::new(
                -100.0 - 150.0 * unit_struct.pos.0 as f32,
                100.0 - 200.0 * unit_struct.pos.1 as f32,
                0.0,
            )
        } else {
            Vec3::new(
                100.0 + 150.0 * unit_struct.pos.0 as f32,
                100.0 - 200.0 * unit_struct.pos.1 as f32,
                0.0,
            )
        };

        // Update text for each unit
        for (mut text, parent, text_type) in texts_query.iter_mut() {
            if parent.get() == parent_entity {
                match text_type {
                    UnitTextMarker::Countdown => {
                        text.sections[0].value = attack.remaining_turns.to_string()
                    }
                    UnitTextMarker::Damage => text.sections[0].value = attack.damage.to_string(),
                    UnitTextMarker::Hp => text.sections[0].value = unit_struct.hp.to_string(),
                }
            }
        }
    }
}

fn process_attacks(
    units: &mut Query<(&mut Unit, &mut Attack, Entity, Option<&IsFriendly>)>,
    friendlies_attacking: bool,
) {
    let mut attackers: Vec<(Mut<'_, Unit>, Mut<'_, Attack>, Entity)> = Vec::new();
    let mut defenders: Vec<(Mut<'_, Unit>, Mut<'_, Attack>, Entity)> = Vec::new();
    for (unit, attack, entity, is_friendly_opt) in units.iter_mut() {
        if friendlies_attacking {
            if is_friendly_opt.is_some() {
                attackers.push((unit, attack, entity));
            } else {
                defenders.push((unit, attack, entity))
            }
        } else {
            if is_friendly_opt.is_some() {
                defenders.push((unit, attack, entity));
            } else {
                attackers.push((unit, attack, entity))
            }
        }
    }
    for (attacker_unit, mut attacker_attack, _attacker_entity) in attackers {
        if attacker_attack.remaining_turns == 0 {
            attacker_attack.remaining_turns = attacker_attack.attack_interval;
            for (ref mut defender_unit, _, _defender_entity) in defenders.iter_mut() {
                if defender_unit.pos.1 == attacker_unit.pos.1 {
                    if defender_unit.pos.0 == 0 {
                        defender_unit.hp -= attacker_attack.damage;
                    }
                }
            }
        }
    }
}

fn deal_damage_system(
    mut units: Query<(&mut Unit, &mut Attack, Entity, Option<&IsFriendly>)>,
    // mut eot_events: EventReader<EndOfTurnEvent>,
) {
    process_attacks(&mut units, true);
    process_attacks(&mut units, false);
}

fn handle_end_of_turn(
    mut friendly_untis: Query<&mut Attack, With<IsFriendly>>,
    mut enemy_units: Query<&mut Attack, Without<IsFriendly>>,
    mut eot_events: EventReader<EndOfTurnEvent>,
    mut cb_manager: ResMut<CombatManager>,
    mut buttons: Query<(&mut CustomButton, &CombatButtonType)>,
) {
    for _ in eot_events.read() {
        for mut unit in &mut friendly_untis {
            unit.countdown();
        }
        for mut unit in &mut enemy_units {
            unit.countdown();
        }

        cb_manager.turn = Turn::Player;
        cb_manager.remaining_energy = cb_manager.max_energy;

        for (mut button, button_type) in buttons.iter_mut() {
            match button_type {
                CombatButtonType::EndTurn => button.enabled = true,
                _ => {}
            }
        }
    }
}

fn spawn_button_on_unit(
    pos: &Transform,
    sprite: &Sprite,
    selected_entity: Entity,
    commands: &mut Commands,
    window: &Window,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(sprite.custom_size.unwrap().x),
                height: Val::Px(sprite.custom_size.unwrap().y),
                left: Val::Px(
                    pos.translation.x + window.width() / 2. - sprite.custom_size.unwrap().x / 2.,
                ),
                top: Val::Px(
                    -pos.translation.y + window.height() / 2. - sprite.custom_size.unwrap().y / 2.,
                ),
                ..default()
            },
            background_color: BackgroundColor(Color::Rgba {
                red: 0.2,
                green: 0.6,
                blue: 0.2,
                alpha: 1.,
            }),
            ..default()
        })
        .insert(CombatThing)
        // .insert(Name::new("Test Button"))
        .insert(CombatButtonType::UnitSelector(selected_entity))
        .insert(CustomButton::new(
            "".to_string(),
            "".to_string(),
            "".to_string(),
            false,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
}

fn provide_player_with_options_system(
    mut commands: Commands,
    friendly_units: Query<(&Transform, &Sprite, Entity), With<IsFriendly>>,
    enemy_units: Query<(&Transform, &Sprite, Entity), Without<IsFriendly>>,
    mut card_events: EventReader<CardEvent>,
    windows: Query<&Window>,
    action_prompt_buttons: Query<(Entity, &CombatButtonType)>,
    mut cb_manager: ResMut<CombatManager>,
    mut buttons: Query<(
        &mut BackgroundColor,
        &mut BorderColor,
        &mut CustomButton,
        &Children,
        &CombatButtonType,
        Entity,
    )>,
    mut text_query: Query<&mut Text>,
) {
    let window = windows.single();
    // debug!("w: {}, h: {}", window.width(), window.height());
    for event in card_events.read() {
        match event {
            CardEvent::CardSelected((selected_card, card_entity)) => {
                // This is to deselect all other cards when one is selected
                cb_manager.selected_card = Some((selected_card.clone(), *card_entity));
                for (mut bg_colour, mut border_colour, mut button_data, children, button_type, _) in
                    buttons.iter_mut()
                {
                    match button_type {
                        CombatButtonType::Card(card_data) => {
                            if card_data == selected_card {
                                let mut text = text_query.get_mut(children[0]).unwrap();
                                text.sections[0].value = button_data.pressed_text.clone();
                                *bg_colour = PRESSED_BUTTON.into();
                                border_colour.0 = Color::WHITE;
                                button_data.toggled = Some(true)
                            }
                        }
                        _ => {}
                    }
                }

                match selected_card.target {
                    Target::Enemy => {
                        for (unit_pos, unit_sprite, entity) in enemy_units.iter() {
                            spawn_button_on_unit(
                                unit_pos,
                                unit_sprite,
                                entity,
                                &mut commands,
                                window,
                            )
                        }
                    }
                    Target::Player => {
                        for (unit_pos, unit_sprite, entity) in friendly_units.iter() {
                            spawn_button_on_unit(
                                unit_pos,
                                unit_sprite,
                                entity,
                                &mut commands,
                                window,
                            )
                        }
                    }
                }
            }
            CardEvent::CardDeselected(opt_entity) => {
                for (button_entity, button_type) in action_prompt_buttons.iter() {
                    match button_type {
                        CombatButtonType::UnitSelector(_) => {
                            commands.entity(button_entity).despawn_recursive()
                        }
                        _ => {}
                    }
                }
                for (mut bg_colour, mut border_colour, mut button, children, button_type, _) in
                    buttons.iter_mut()
                {
                    match button_type {
                        CombatButtonType::Card(_) => {
                            button.toggled = Some(false);
                            cb_manager.selected_card = None;
                            *bg_colour = NORMAL_BUTTON.into();
                            border_colour.0 = Color::BLACK;
                            let mut text = text_query.get_mut(children[0]).unwrap();
                            text.sections[0].value = button.off_text.clone()
                        }
                        _ => {}
                    }
                }
                if let Some(entity) = opt_entity {
                    for (_, _, _, _, _, button_entity) in buttons.iter_mut() {
                        if button_entity == *entity {
                            commands.entity(*entity).despawn_recursive()
                        }
                    }
                }
            }
        }
    }
}

fn remove_dead_units(mut commands: Commands, units: Query<(&Unit, Entity)>) {
    for (unit, entity) in units.iter() {
        if unit.hp <= 0 {
            commands.entity(entity).despawn_recursive()
        }
    }
}

fn end_combat(
    enemy_units: Query<Entity, (With<Unit>, Without<IsFriendly>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if enemy_units.is_empty() {
        next_state.set(GameState::Map);
    }
}

fn clean_up(mut commands: Commands, combat_things: Query<Entity, With<CombatThing>>) {
    for combat_thing in &combat_things {
        commands.entity(combat_thing).despawn_recursive()
    }
}
