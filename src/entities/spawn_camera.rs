use bevy::prelude::*;

pub fn spawn_camera(
    commands: &mut Commands,
) {
    commands.spawn(Camera2d);
}