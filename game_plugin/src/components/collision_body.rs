use bevy::prelude::Component;
use std::fmt::{self, Display, Formatter};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct CollisionBody {
    pub width: u16,
    pub height: u16,
}

impl Display for CollisionBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}
