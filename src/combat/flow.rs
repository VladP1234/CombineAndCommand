use crate::*;

pub fn spawn_stuff(mut commands: Commands, player: Res<Player>) {
    spawn_unit(
        &mut commands,
        Unit::new(10, (0, 0)),
        true,
        Attack::new(1, 2),
    );
    spawn_unit(
        &mut commands,
        Unit::new(10, (0, 0)),
        false,
        Attack::new(1, 1),
    );
    let hand_node: Entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(30.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                top: Val::Percent(70.0),
                ..default()
            },
            background_color: Color::rgba(0., 0., 0., 0.).into(),
            ..default()
        })
        .insert(CombatThing)
        .insert(Name::new("Hand Node"))
        .id();

    commands.insert_resource(CombatManager::new(player.deck.clone(), 3, hand_node));

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
        .insert(Name::new("End Turn Node"))
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
                .insert(Name::new("End Turn Button"))
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

pub fn preflight(mut commands: Commands, mut cb_manager: ResMut<CombatManager>) {
    debug!("here");
    for _ in 0..3 {
        let attempted_drawn_card = cb_manager.draw();
        if let Some(drawn_card) = attempted_drawn_card {
            let card_entity = make_card(&mut commands, &drawn_card);
            commands
                .entity(cb_manager.hand_node)
                .insert_children(0, &[card_entity]);
        }
    }
}

pub fn update_unit_ui(
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

pub fn process_attacks(
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

pub fn deal_damage_system(
    mut units: Query<(&mut Unit, &mut Attack, Entity, Option<&IsFriendly>)>,
    // mut eot_events: EventReader<EndOfTurnEvent>,
) {
    process_attacks(&mut units, true);
    process_attacks(&mut units, false);
}

pub fn handle_end_of_turn(
    mut friendly_untis: Query<&mut Attack, With<IsFriendly>>,
    mut enemy_units: Query<&mut Attack, Without<IsFriendly>>,
    mut eot_events: EventReader<EndOfTurnEvent>,
    mut cb_manager: ResMut<CombatManager>,
    mut buttons: Query<(&mut CustomButton, &CombatButtonType)>,
    // mut nodes: Query<Entity, With<(CombatThing)>>,
    mut commands: Commands,
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
        let attempted_drawn_card = cb_manager.draw();
        if let Some(drawn_card) = attempted_drawn_card {
            let card_entity = make_card(&mut commands, &drawn_card);
            commands
                .entity(cb_manager.hand_node)
                .insert_children(0, &[card_entity]);
        }
    }
}

pub fn remove_dead_units(mut commands: Commands, units: Query<(&Unit, Entity)>) {
    for (unit, entity) in units.iter() {
        if unit.hp <= 0 {
            commands.entity(entity).despawn_recursive()
        }
    }
}

pub fn end_combat(
    enemy_units: Query<Entity, (With<Unit>, Without<IsFriendly>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if enemy_units.is_empty() {
        next_state.set(GameState::Map);
    }
}

pub fn clean_up(mut commands: Commands, combat_things: Query<Entity, With<CombatThing>>) {
    for combat_thing in &combat_things {
        commands.entity(combat_thing).despawn_recursive()
    }
}
