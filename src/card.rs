use bevy::{ecs::component::Component, reflect::Reflect};

#[derive(Reflect, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Effect {
    BonusDamage(i32),
    BonusHealth(i32),
    BonusCountdown(i32),
    DealDamage(i32),
}

#[derive(Reflect, Clone, PartialEq, Component, Debug)]
pub struct Card {
    pub effects: Vec<Effect>,
    pub cost: i32,
    pub description: String,
}

pub trait ToEffectVec {
    fn to_effect_vec(self) -> Vec<Effect>;
}

impl ToEffectVec for Effect {
    fn to_effect_vec(self) -> Vec<Effect> {
        vec![self]
    }
}

impl ToEffectVec for Vec<Effect> {
    fn to_effect_vec(self) -> Vec<Effect> {
        self
    }
}

impl Card {
    pub fn new<T: ToEffectVec + Clone>(effects: T, cost: i32) -> Card {
        Card {
            effects: effects.clone().to_effect_vec(),
            cost,
            description: make_description(effects.to_effect_vec()),
        }
    }
}

pub fn make_description(effects: Vec<Effect>) -> String {
    let mut descriptions = Vec::new();

    for effect in effects {
        let description = match effect {
            Effect::BonusDamage(amount) => {
                if amount >= 0 {
                    format!("increase the attack of a unit by {}", amount)
                } else {
                    format!("decrease the attack of a unit by {}", -amount)
                }
            }
            Effect::BonusHealth(amount) => {
                if amount >= 0 {
                    format!("increase the health max by {}", amount)
                } else {
                    format!("decrease the health max by {}", -amount)
                }
            }
            Effect::BonusCountdown(amount) => {
                if amount >= 0 {
                    format!("reduce the countdown by {}", amount)
                } else {
                    format!("increase the countdown by {}", -amount)
                }
            }
            Effect::DealDamage(amount) => {
                if amount >= 0 {
                    format!("deal {} damage", amount)
                } else {
                    format!("heal {} damage", -amount)
                }
            }
        };
        descriptions.push(description);
    }

    descriptions.join(" and ")
}
