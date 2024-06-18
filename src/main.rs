use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::{
    ResourceInspectorPlugin, StateInspectorPlugin, WorldInspectorPlugin,
};
use bevy_mod_picking::DefaultPickingPlugins;
use combine_and_command::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
                // No clue what any of this means, but I know that this makes debug!() work
                LogPlugin {
                    level: Level::TRACE,
                    filter:
                        "wgpu=warn,bevy_ecs=info,winit=info,naga=info,bevy_app=info,bevy_winit=info\
                    ,bevy_render=info,bevy_core=info,gilrs=info,bevy_picking_core=warn"
                            .to_string(),
                },
            ),
            MapPlugin,
            DefaultPickingPlugins,
            CombatPlugin,
            RestSitePlugin,
            BasePlugin,
            UtilsPlugin,
            // Debug stuff
            WorldInspectorPlugin::new(),
            StateInspectorPlugin::<GameState>::default(),
            // ResourceInspectorPlugin::<MapManager>::default(),
            // ResourceInspectorPlugin::<CombatManager>::default(),
            ResourceInspectorPlugin::<Player>::default(),
        ))
        .register_type::<MapManager>()
        .register_type::<SelectedCard>()
        .run();
}
