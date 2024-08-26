use bevy::ecs::schedule::States;
use bevy::prelude::*;
pub mod map;
pub use map::*;
pub mod rest_sites;
pub use rest_sites::*;
pub mod combat;
pub use combat::*;
pub mod card;
pub use card::*;
pub mod base;
pub use base::*;
pub mod utils;
pub use utils::*;
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default, Reflect)]
pub enum GameState {
    #[default]
    HomeBase,
    Map,
    Combat,
    Merchant,
    RestSite,
}

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.55, 0.55, 0.55);
pub const START_TILE: (i32, i32) = (2, 8);
pub const COMBATS_DATA: &str = include_str!("data/combats_data.json");

#[derive(Component, Reflect)]
pub struct CustomButton {
    enabled: bool,
    toggled: Option<bool>,
    off_text: String,
    hover_text: String,
    pressed_text: String,
}

impl CustomButton {
    fn new(
        off_text: String,
        hover_text: String,
        pressed_text: String,
        is_toggleable: bool,
    ) -> CustomButton {
        CustomButton {
            enabled: true,
            off_text,
            hover_text,
            pressed_text,
            toggled: if is_toggleable { Some(false) } else { None },
        }
    }
}
#[derive(Resource, Default, Reflect)]
pub struct Player {
    pub deck: Vec<Card>,
    pub units: Vec<(Unit, Attack)>,
}

impl Player {
    pub fn new(deck: Vec<Card>, units: Vec<(Unit, Attack)>) -> Player {
        Player { deck, units }
    }
}

pub fn json_from_str<T>(data: &str) -> Result<T, serde_json::Error>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(data)
}
