// use bevy::log::Level;
// use bevy::log::LogPlugin;
use bevy::prelude::*;
// use bevy_inspector_egui::quick::{
//     ResourceInspectorPlugin, StateInspectorPlugin, WorldInspectorPlugin,
// };
use bevy_mod_picking::DefaultPickingPlugins;
use combine_and_command::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, // makes debug!() function, uncomment to have your terminal flooded. Make sure to move the comma to after .set()
            // .set(
            //     LogPlugin {
            //         level: Level::TRACE,
            //         filter:
            //             "wgpu=warn,bevy_ecs=info,winit=info,naga=info,bevy_app=info,bevy_winit=info\
            //         ,bevy_render=info,bevy_core=info,gilrs=info,bevy_picking_core=warn"
            //                 .to_string(),
            //     },
            // )
            MapPlugin,
            DefaultPickingPlugins,
            CombatPlugin,
            RestSitePlugin,
            BasePlugin,
            UtilsPlugin,
            // Debug stuff, uncomment at your own risk
            // WorldInspectorPlugin::new(),
            // StateInspectorPlugin::<GameState>::default(),
            // ResourceInspectorPlugin::<MapManager>::default(),
            // ResourceInspectorPlugin::<CombatManager>::default(),
            // ResourceInspectorPlugin::<Player>::default(),
        ))
        // Also for debugging
        // .register_type::<MapManager>()
        // .register_type::<SelectedCard>()
        .run();
}
