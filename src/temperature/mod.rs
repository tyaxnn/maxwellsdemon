use bevy::prelude::*;

use crate::entities::{spawn_ball::Velocity, ScoreText};

pub fn calculate_temperature(
    query : Query<(&Transform, &Velocity)>,

    score_text: Single<Entity, With<ScoreText>>,
    mut writer: TextUiWriter,
){
    let mut left_e = 0.;
    let mut right_e = 0.;

    let mut left_n = 0;
    let mut right_n = 0;
 
    for (transform, vel) in query.iter() {

        let speed = vel.length();
        let energy = speed * speed / 100.;
        if transform.translation.x < 0.{
            left_e += energy;
            left_n += 1;
        }
        else{
            right_e += energy;
            right_n += 1;
        }
    }

    let left_t = left_e / left_n as f32;
    let right_t = right_e / right_n as f32;

    let carnot = round3(carnot(left_t, right_t));

    *writer.text(*score_text, 0) = format!("L_T : {} score : {}, R_T : {}",round3(left_t),carnot,round3(right_t));
}

fn round3(x : f32) -> f32 {
    (x * 1000.).round()/1000.
}

fn carnot(x : f32, y : f32) -> f32{
    if x >y {
        1. - y/x
    }
    else {
        1. - x/y
    }
}