use bevy::prelude::*;


//=========================//
mod entities;
use entities::EntitiesPlugin;

mod inputs;
use inputs::InputHandlePlugin;

mod physics;
use physics::PhysicsHandlePlugin;

mod temperature;
use temperature::calculate_temperature;
//=========================//


pub fn run() {
    App::new()
        .insert_resource(AmbientLight{
            color: Color::srgb(1.0, 0.9, 0.8),
            brightness: 2000.4,
        })
        .add_plugins((
            DefaultPlugins,
            EntitiesPlugin,
            InputHandlePlugin,
            PhysicsHandlePlugin,
            
        ))
        .add_systems(Update, calculate_temperature)
        .run();
}

