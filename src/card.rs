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
}

impl Card {
    pub fn new(target: Target, effect: Effect, cost: i32) -> Card {
        Card {
            target,
            effect,
            cost,
        }
    }
}
