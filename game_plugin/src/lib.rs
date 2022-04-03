pub mod components;
pub mod resources;

use bevy::ecs::schedule::StateData;
use bevy::log;
use bevy::{prelude::*, sprite::SpriteBundle};
use components::*;
use std::f32::consts::PI;

#[cfg(feature = "debug")]
use bevy_inspector_egui::InspectableRegistry;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

pub struct GamePlugin<T> {
    pub running_state: T,
}

impl<T: StateData> Plugin for GamePlugin<T> {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        {
            let mut registry = app
                .world
                .get_resource_or_insert_with(InspectableRegistry::default);
            registry.register::<Player>();
            registry.register::<CollisionBody>();
        }

        app.add_system_set(
            SystemSet::on_enter(self.running_state.clone())
                .with_system(Self::create_lifemap)
                .with_system(Self::create_shit),
        )
        .add_system_set(
            SystemSet::on_update(self.running_state.clone())
                .with_system(Self::update_shit)
                .with_system(Self::update_player),
        );
    }
}

impl<T> GamePlugin<T> {
    pub fn create_lifemap() {}

    pub fn create_shit(mut commands: Commands, asset_server: Res<AssetServer>) {
        // TODO
        // create walls and player to test collisions
        //
        //
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("ship.png"),
                sprite: Sprite {
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., 0., 5.),
                ..Default::default()
            })
            .insert(GlobalTransform::default())
            .insert(Player {
                image_angle: 0.0,
                vsp: 0.0,
                hsp: 0.0,
            })
            .insert(Name::new("Player"));

        let board_entity = commands
            .spawn()
            .insert(Name::new("LifeMap"))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: asset_server.load("test.png"),
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(Vec2::new(256., 256.)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(256. / 2., 256., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));
            });
    }

    pub fn update_shit(mut commands: Commands) {
        // TODO
        // update shit
    }

    pub fn update_player(mut q: Query<(&mut Player, &mut Transform)>, keys: Res<Input<KeyCode>>) {
        let (mut player, mut transform) = q.single_mut();

        if keys.pressed(KeyCode::Left) {
            player.image_angle += 0.1;
            transform.rotation = Quat::from_rotation_z(player.image_angle);
        } else if keys.pressed(KeyCode::Right) {
            player.image_angle -= 0.1;
            transform.rotation = Quat::from_rotation_z(player.image_angle);
        }

        // somehow have to flip plus and minus
        let rad = -(player.image_angle % (PI * 2.));

        let mut add_vsp = 0.;
        let mut add_hsp = 0.;

        if keys.just_pressed(KeyCode::Up) {
            add_vsp = rad.sin() * AMOUNT;
            add_hsp = rad.cos() * AMOUNT;
        }

        const AMOUNT: f32 = 0.6;

        player.vsp += add_vsp;
        player.hsp += add_hsp;
        player.vsp *= FRICTION;
        player.hsp *= FRICTION;

        const FRICTION: f32 = 0.97;

        transform.translation.x += player.vsp;
        transform.translation.y += player.hsp;
    }
}
