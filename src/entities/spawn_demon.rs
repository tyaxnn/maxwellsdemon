use bevy::prelude::*;

#[derive(Component)]
pub struct Akuma;

pub fn spawn_demon(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
){


    let demon = asset_server.load("pic/demon.png"); 
    let maxwell = asset_server.load("pic/maxwell.png"); 

    let fukidashi = asset_server.load("pic/fukidashi_bw03.png"); 

    commands.spawn((Sprite{
        image : demon,
        ..default()
    },
    Transform::from_xyz(0.0, 180.0, 100.0).with_scale(Vec3::new(0.5,0.5,0.5)),
    Akuma,
    ));

    commands.spawn((Sprite{
        image : maxwell,
        ..default()
    },
    Transform::from_xyz(0.0, 180.0, 10.0).with_scale(Vec3::new(0.5,0.5,0.5)),
    Akuma,
    ));

    commands.spawn((Sprite{
        image : fukidashi,
        ..default()
    },
    Transform::from_xyz(180.0, 245.0, 3.0).with_scale(Vec3::new(0.5,0.5,0.5)),
    ));
}