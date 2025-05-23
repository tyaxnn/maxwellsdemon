use bevy::prelude::*;

use crate::entities::{
    spawn_wall::RemovableWall,
    spawn_demon::Akuma,
    spawn_ball::spawn_balls
};

use crate::{GameState, StartGame};


pub fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit.send(AppExit::Success); 
    }
}

pub fn remove_gap(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut RemovableWall)>,
){     

    if keys.just_pressed(KeyCode::Space){
        for (mut transform, mut open) in query.iter_mut(){
            match open.0{
                true => {
                    transform.translation = Vec3::new(0.,0.,0.);
                    open.0 = false;
                }
                false => {
                    transform.translation = Vec3::new(0.,10000.,0.);
                    open.0 = true;
                }
            }
            
        }

        
    }

}

pub fn akuma_moves(
    keys: Res<ButtonInput<KeyCode>>,
    mut akuma_query: Query<&mut Transform, With<Akuma>>,
){
    if keys.just_pressed(KeyCode::Space){
        for mut transform_akuma in akuma_query.iter_mut(){
            transform_akuma.translation.y += 10.0
        }
    }

    if keys.just_released(KeyCode::Space){
        for mut transform_akuma in akuma_query.iter_mut(){
            transform_akuma.translation.y -= 10.0
        }
    }
}

pub fn change_state_to_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut start_time: ResMut<StartGame>,
    time: Res<Time>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
){
    if keys.just_pressed(KeyCode::Space){
        game_state.set(GameState::Game);
        start_time.time = time.elapsed_secs();
        start_time.score = 0.0;

        spawn_balls(commands, meshes, materials);
    }
}

pub fn change_state_to_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
){
    if keys.just_pressed(KeyCode::Enter){
        game_state.set(GameState::Menu);
    }
}
