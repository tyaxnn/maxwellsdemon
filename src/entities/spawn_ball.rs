use bevy::prelude::*;
use rand::{
    random_bool,
    random_range,
};
use std::f32::consts::TAU;

use crate::entities::spawn_wall::{
    LEFT_WALL,
    RIGHT_WALL,
    TOP_WALL,
    BOTTOM_WALL,
};

pub const BALL_DIAMETER: f32 = 8.;

pub const BALL_MAX_SPEED: f32 = 200.;
pub const BALL_MIN_SPEED: f32 = 40.;
const BALL_NUM: usize = 700;

struct BallProperty{
    ini_dir : Vec2,
    ini_coord : Vec3,
    speed : f32,
}

impl BallProperty{
    fn random() -> Self {

        let pos_x = if random_bool(0.5){
            LEFT_WALL / 2.

        }else{
            RIGHT_WALL /2.
        };

        let pos_y = (TOP_WALL + BOTTOM_WALL) / 2.;

        let ini_coord = Vec3::new(pos_x,pos_y,0.);

        let angle = random_range((0.)..TAU);

        let ini_dir = Vec2::new(angle.cos(), angle.sin()).normalize();

        let speed = random_range(BALL_MIN_SPEED..BALL_MAX_SPEED);
        
        BallProperty{
            ini_dir,
            ini_coord,
            speed,
        }

    }
}


#[derive(Component)]
pub struct Ball;


#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);



pub fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) { 

    for _ in 0..BALL_NUM{
        let property = BallProperty::random();

        let color = {
            let speed_clamped = (property.speed - BALL_MIN_SPEED) / (BALL_MAX_SPEED - BALL_MIN_SPEED);
            Color::srgb(speed_clamped, 0.0, 1.0 - speed_clamped)
        };

        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(color)),
            Transform::from_translation(property.ini_coord)
                .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
            Ball,
            Velocity(property.ini_dir * property.speed),
        ));
    } 
}

pub fn despawn_balls(
    query: Query<Entity, With<Ball>>,
    mut commands: Commands,
){
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
