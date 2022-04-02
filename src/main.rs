use bevy::log;
use bevy::prelude::*;
use game_plugin::GamePlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    UpgradeStore,
    Paused,
}

fn main() {
    // println!("{:?}"LifeTile::new_tile('r') );
    let mut app = App::new();

    // Window setup
    app.insert_resource(WindowDescriptor {
        title: "Game".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    })
    // Bevy default plugins
    .add_state(AppState::InGame)
    .add_plugin(GamePlugin {
        running_state: AppState::InGame,
    })
    .add_plugins(DefaultPlugins)
    .add_system(state_handler);

    // Startup system (cameras)
    app.add_startup_system(camera_setup);
    // Debug hierarchy inspector
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    // Run the app
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {}
