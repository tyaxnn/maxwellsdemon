use bevy::prelude::*;

pub mod spawn_wall;
pub mod spawn_camera;
pub mod spawn_ball;
pub mod spawn_demon;
pub mod spawn_text;

use spawn_wall::spawn_walls;
use spawn_camera::spawn_camera;
use spawn_demon::spawn_demon;
use spawn_text::{spawn_remaining_time, spawn_score, spawn_left_temperature, spawn_right_temperature};



pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    spawn_walls(&mut commands);
    spawn_camera(&mut commands);
    spawn_demon(&mut commands, &asset_server);
    spawn_score(&mut commands, &asset_server);
    spawn_remaining_time(&mut commands, &asset_server);
    spawn_left_temperature(&mut commands, &asset_server);
    spawn_right_temperature(&mut commands, &asset_server);
}
