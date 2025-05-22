use bevy::prelude::*;


//=========================//
mod entities;
use entities::{spawn_text::RemainingTimeText, EntitiesPlugin, spawn_ball::{Ball,despawn_balls}, spawn_text::ScoreText};

mod inputs;
use inputs::InputHandlePlugin;

mod physics;
use physics::PhysicsHandlePlugin;

mod temperature;
use temperature::calculate_temperature;
//=========================//

const TIME_LIMIT : f32 = 99.;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
    Result
}

#[derive(Resource)]
pub struct StartGame{
    pub time : f32,
    pub score : f32,
}



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
        .insert_resource(StartGame{time : 0.0, score : 0.0})
        .init_state::<GameState>()
        .add_systems(Update, calculate_temperature.run_if(in_state(GameState::Game)))
        .add_systems(Update, go_to_result.run_if(in_state(GameState::Game)))
        .add_systems(Update, display_score.run_if(in_state(GameState::Result)))
        .add_systems(Update, update_remaining_time)
        .run();
}

fn go_to_result(
    mut game_state: ResMut<NextState<GameState>>,
    start_info: Res<StartGame>,
    time: Res<Time>,
    commands: Commands,
    query: Query<Entity, With<Ball>>,
){
    let elapsed_time = time.elapsed_secs() - start_info.time;

    if elapsed_time > TIME_LIMIT{
        game_state.set(GameState::Result);
        despawn_balls(query, commands);
    }
}

fn update_remaining_time(
    remaining_time_text: Single<Entity, With<RemainingTimeText>>,
    mut writer: TextUiWriter,
    start_time: ResMut<StartGame>,
    time: Res<Time>,
    game_state: Res<State<GameState>>,
){  
    match game_state.get(){
        GameState::Game => {
            *writer.text(*remaining_time_text, 0) = format!("{}",(TIME_LIMIT - (time.elapsed_secs() - start_time.time)).round());
        }
        _ => {
            *writer.text(*remaining_time_text, 0) = "".to_string();
        }
    }
    
}

fn display_score(

    mut writer: TextUiWriter,
    score_text: Single<Entity, With<ScoreText>>,
    start_info: ResMut<StartGame>,
    game_state: Res<State<GameState>>,
    
){
    match game_state.get(){
        GameState::Game => {
            *writer.text(*score_text, 0) = format!("score : {}",start_info.score);
        }
        GameState::Result => {
            *writer.text(*score_text, 0) = format!("Finish! score : {}",start_info.score);
        }
        GameState::Menu => {
            *writer.text(*score_text, 0) = format!("press space to start");
        }
    }
    
    

}

