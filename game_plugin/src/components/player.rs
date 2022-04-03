use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Component)]
pub struct Player {
    pub image_angle: f32,
    pub hsp: f32,
    pub vsp: f32,
}
