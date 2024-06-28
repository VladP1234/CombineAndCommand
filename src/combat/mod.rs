pub mod flow;
pub mod player_actions;
use crate::*;
pub use flow::*;
pub use player_actions::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Combat), spawn_stuff)
            .add_systems(Update, preflight.run_if(resource_added::<CombatManager>()))
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
    deck: Vec<Card>,
    selected_card: Option<(Card, Entity)>,
    draw_pile: Vec<Card>,
    hand_node: Entity,
    discard_pile: Vec<Card>,
}

impl CombatManager {
    pub fn new(deck: Vec<Card>, energy: i32, hand_node: Entity) -> CombatManager {
        CombatManager {
            turn: Turn::Player,
            remaining_energy: energy,
            max_energy: energy,
            selected_card: None,
            hand_node,
            deck: deck.clone(),
            draw_pile: deck,
            discard_pile: Vec::new(),
        }
    }
    pub fn draw(&mut self) -> Option<Card> {
        if let Some(top_card) = self.draw_pile.pop() {
            Some(top_card)
        } else {
            self.shuffle();
            if self.draw_pile.len() > 0 {
                self.draw()
            } else {
                None
            }
        }
    }
    pub fn shuffle(&mut self) {
        self.draw_pile.append(&mut self.discard_pile);
        let mut rng = thread_rng();
        self.draw_pile.shuffle(&mut rng)
    }
}
#[derive(Component)]
pub struct CombatThing;

#[derive(Component, Reflect, Debug)]
pub struct Unit {
    max_hp: i32,
    hp: i32,
    pos: (i32, i32),
}

impl Unit {
    pub fn new(hp: i32, pos: (i32, i32)) -> Unit {
        Unit {
            max_hp: hp,
            hp,
            pos,
        }
    }
}

#[derive(Component)]
pub struct IsFriendly;

#[derive(Component, Reflect, Debug)]
pub struct Attack {
    pub attack_interval: i32,
    pub remaining_turns: i32,
    pub damage: i32,
    // TODO: Damage Type
}
impl Attack {
    pub fn new(attack_interval: i32, damage: i32) -> Attack {
        Attack {
            attack_interval,
            remaining_turns: attack_interval,
            damage,
        }
    }
    pub fn countdown(&mut self) {
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
pub enum UnitTextMarker {
    Countdown,
    Damage,
    Hp,
}

#[derive(Event)]
pub struct EndOfTurnEvent;

#[derive(Event)]
pub enum CardEvent {
    CardSelected((Card, Entity)),
    CardDeselected(Option<Entity>),
}

pub fn spawn_unit(commands: &mut Commands, unit: Unit, is_friendly: bool, attack: Attack) {
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
                .insert(UnitTextMarker::Countdown)
                .insert(Name::new("Attack Countdown"));
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
                .insert(UnitTextMarker::Hp)
                .insert(Name::new("HP"));
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
                .insert(UnitTextMarker::Damage)
                .insert(Name::new("Damage"));
        });

    if is_friendly {
        mod_unit.insert(IsFriendly);
    }
}

pub fn make_card(commands: &mut Commands, card: &Card) -> Entity {
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
        .insert(CombatButtonType::Card(card.clone()))
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
                ..default()
            });
        });
    card.id()
}
