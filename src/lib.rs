use bevy::ecs::schedule::States;
use bevy::prelude::*;
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default, Reflect)]
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

#[derive(Component, Reflect)]
pub struct CustomButton {
    enabled: bool,
    toggled: Option<bool>,
    off_text: String,
    hover_text: String,
    pressed_text: String,
}

#[allow(dead_code)]
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
