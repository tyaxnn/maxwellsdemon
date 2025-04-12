use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::entities::{
    spawn_ball::{Velocity,Ball,BALL_DIAMETER},
    spawn_wall::Collider
};


pub struct PhysicsHandlePlugin;


#[derive(Event, Default)]
struct CollisionEvent;


impl Plugin for PhysicsHandlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            apply_velocity,
            check_for_collisions
        )).add_event::<CollisionEvent>()
        ;
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn check_for_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (mut ball_velocity, ball_transform) in ball_query.iter_mut(){

        let clamp = ball_velocity.length()  / 1000.;

        for collider_transform in &collider_query {
            let collision = ball_collision(
                BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER * (0.5 + clamp * 1.)),
                Aabb2d::new(
                    collider_transform.translation.truncate(),
                    collider_transform.scale.truncate() / 2.,
                ),
            );

            if let Some(collision) = collision {
                // Sends a collision event so that other systems can react to the collision
                collision_events.send_default();

                // Reflect the ball's velocity when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // Reflect only if the velocity is in the opposite direction of the collision
                // This prevents the ball from getting stuck inside the bar
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                }

                // Reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                // Reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }
        }
    }
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

// Returns `Some` if `ball` collides with `bounding_box`.
// The returned `Collision` is the side of `bounding_box` that `ball` hit.
fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

