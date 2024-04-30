use bevy::reflect::Reflect;

#[derive(Reflect, Clone, PartialEq)]
pub enum Target {
    Enemy,
    Player,
}

#[derive(Reflect, Clone, PartialEq)]
pub enum Effect {
    BonusDamage(i32),
    BonusHealth(i32),
    BonusCountdown(i32),
}

#[derive(Reflect, Clone, PartialEq)]
pub struct Card {
    pub target: Target,
    pub effect: Effect,
    pub cost: i32,
    pub description: String,
}

impl Card {
    pub fn new(target: Target, effect: Effect, cost: i32, description: String) -> Card {
        Card {
            target,
            effect,
            cost,
            description,
        }
    }
}
