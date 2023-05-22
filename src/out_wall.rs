use bevy::prelude::*;
use bevy::window::Window;

use crate::{config, wall, wall_location};

#[derive(Component, Clone, Copy)]
pub struct OutWall;

pub fn spawn(parent: &mut ChildBuilder, window: &Window, bundle: impl Bundle + Copy) {
    let size = Vec2 {
        x: window.width(),
        y: window.height(),
    };
    let thickness = config::Wall::THICKNESS;
    wall::spawn(
        parent,
        wall_location::WallLocation::Left,
        size,
        thickness,
        (bundle, OutWall),
    );
    wall::spawn(
        parent,
        wall_location::WallLocation::Right,
        size,
        thickness,
        (bundle, OutWall),
    );
    wall::spawn(
        parent,
        wall_location::WallLocation::Top,
        size,
        thickness,
        (bundle, OutWall),
    );
    wall::spawn(
        parent,
        wall_location::WallLocation::Bottom,
        size,
        thickness,
        (bundle, OutWall),
    );
}
