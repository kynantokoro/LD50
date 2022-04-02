use bevy::log;
use bevy::prelude::*;
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin};
use game_plugin::GamePlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

const WIDTH: f32 = 256.0;
const HEIGHT: f32 = 256.0;

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
    .add_plugin(PixelCameraPlugin)
    .add_plugin(PixelBorderPlugin {
        color: Color::rgb(0.8, 0.1, 0.1),
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .add_startup_system(setup.system().label("setup"))
    .add_system(state_handler);

    // Debug hierarchy inspector
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    // Run the app
    app.run();
}

fn setup(mut commands: Commands, time: Res<Time>) {
    commands.spawn_bundle(PixelCameraBundle::from_resolution(
        WIDTH as i32,
        HEIGHT as i32,
    ));
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {}
