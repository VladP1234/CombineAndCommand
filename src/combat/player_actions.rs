use crate::*;

pub fn button_system(
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
                                    for effect in &card.effects {
                                        match effect {
                                            Effect::BonusDamage(damage) => {
                                                attack.damage = (attack.damage + damage).max(0);
                                            }
                                            Effect::BonusHealth(health) => unit.hp += health,
                                            Effect::BonusCountdown(countdown) => {
                                                attack.remaining_turns += countdown
                                            }
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

pub fn spawn_button_on_unit(
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
        .insert(Name::new("Target Picker"))
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

pub fn provide_player_with_options_system(
    mut commands: Commands,
    units: Query<(&Transform, &Sprite, Entity), With<Unit>>,
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

                for (unit_pos, unit_sprite, entity) in units.iter() {
                    spawn_button_on_unit(unit_pos, unit_sprite, entity, &mut commands, window)
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
                    for (_, _, _, _, button_type, button_entity) in buttons.iter_mut() {
                        if button_entity == *entity {
                            match button_type {
                                CombatButtonType::Card(card) => {
                                    cb_manager.discard_pile.push(card.clone())
                                }
                                _ => {}
                            }
                            commands.entity(*entity).despawn_recursive()
                        }
                    }
                }
            }
        }
    }
}
