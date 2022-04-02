pub mod components;
pub mod resources;

use bevy::ecs::schedule::StateData;
use bevy::log;
use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::InspectableRegistry;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

pub struct GamePlugin<T> {
    pub running_state: T,
}

impl<T: StateData> Plugin for GamePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(self.running_state.clone()).with_system(Self::create_lifemap),
        );

        #[cfg(feature = "debug")]
        {
            let mut registry = app
                .world
                .get_resource_or_insert_with(InspectableRegistry::default);
            // registering custom component to be able to edit it in inspector
            app.register_inspectable::<Coordinates>();
            app.register_inspectable::<BombNeighbor>();
            app.register_inspectable::<Bomb>();
            app.register_inspectable::<Uncover>();
        }
    }
}

impl<T> GamePlugin<T> {
    pub fn create_lifemap() {

        // TODO
        // create an 2d array
        //
        //
        //
        // output [[0, 0, 0, 1], [0, 1, 1, 1], [], []],
        //
        // TODO
        // draw array to console
        //
        // itr on the array and spawn tiles
        //
        // itr on tiles and spawn walls
        //
        // spawn player, goal, enegy, coins
    }
}
