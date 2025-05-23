use bevy::prelude::*;

use std::fs;
use std::path::Path;

//=========================//
mod entities;
use entities::{spawn_ball::{despawn_balls, Ball}, spawn_text::{DemonSerif, LeftTemperatureText, RemainingTimeText, RightTemperatureText, ScoreText, Top10text}, EntitiesPlugin};

mod inputs;
use inputs::InputHandlePlugin;

mod physics;
use physics::PhysicsHandlePlugin;

mod temperature;
use scores::OldScores;
use temperature::calculate_temperature;
mod scores;
//=========================//

const TIME_LIMIT : f32 = 99.;

const SCORE_FILE: &str = "assets/high_scores.json";

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
        .insert_resource({
            let top10 ={

                let mut scores = [0.0;10];

                if Path::new(SCORE_FILE).exists() {
                    if let Ok(data) = fs::read_to_string(SCORE_FILE) {
                        if let Ok(read_scores) = serde_json::from_str::<[f32;10]>(&data) {
                            scores = read_scores;
                        }
                    }
                }

                scores
                
            };

            OldScores{top10}
        }
        )
        .init_state::<GameState>()
        .add_systems(Update, calculate_temperature.run_if(in_state(GameState::Game)))
        .add_systems(Update, go_to_result.run_if(in_state(GameState::Game)))
        .add_systems(Update, display_score.run_if(in_state(GameState::Result)))
        .add_systems(Update, display_score.run_if(in_state(GameState::Menu)))
        .add_systems(Update, update_remaining_time)
        .run();
}

fn go_to_result(
    mut game_state: ResMut<NextState<GameState>>,
    start_info: Res<StartGame>,
    time: Res<Time>,
    commands: Commands,
    query: Query<Entity, With<Ball>>,
    mut old_scores: ResMut<OldScores>,

    mut writer: TextUiWriter,
    akuma_serif: Single<Entity, With<DemonSerif>>,
    score_text: Single<Entity,With<Top10text>>
){
    let elapsed_time = time.elapsed_secs() - start_info.time;

    if elapsed_time > TIME_LIMIT{
        game_state.set(GameState::Result);
        despawn_balls(query, commands);

        old_scores.top10q(start_info.score);


        if let Ok(json) = serde_json::to_string(&old_scores.top10) {
            fs::write(SCORE_FILE, json).expect("Failed to write score file");
        }

        if start_info.score > 1400.{
            *writer.text(*akuma_serif, 0) = format!("きみは\nたくえつでーもん\nだ");
        }
        else if start_info.score > 1200.{
            *writer.text(*akuma_serif, 0) = format!("きみは\nはいぱーでーもん\nだ");
        }
        else if start_info.score > 900.{
            *writer.text(*akuma_serif, 0) = format!("きみは\nますたーおぶでーもん\nだ");
        }
        else if start_info.score > 600.{
            *writer.text(*akuma_serif, 0) = format!("きみは\nきんぐでーもん\nだ");
        }
        else if start_info.score > 300.{
            *writer.text(*akuma_serif, 0) = format!("きみは\nふつうのでーもん\nだ");
        }
        else if start_info.score > 100.{
            *writer.text(*akuma_serif, 0) = format!("きみは\nでーもんみならい\nだ");
        }
        else{
            *writer.text(*akuma_serif, 0) = format!("おいおい！\nちゃんとしごとしろ！");
        }

        *writer.text(*score_text, 0) = old_scores.display();
    }
    else if elapsed_time > TIME_LIMIT - 10.{
        *writer.text(*akuma_serif, 0) = format!("あとすこし\nがんばろう");
    }
    else if elapsed_time > TIME_LIMIT - 40.{
        *writer.text(*akuma_serif, 0) = format!("きをぬくな！");
    }
    else if elapsed_time > 10.{
        *writer.text(*akuma_serif, 0) = format!("だいにほうそくを\nやぶるんだ！");
    }
    else{
        *writer.text(*akuma_serif, 0) = format!("すぺーすで\nかべをきりかえるぞ");
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
    left_temperature_text: Single<Entity, With<LeftTemperatureText>>,
    right_temperature_text: Single<Entity, With<RightTemperatureText>>,
    start_info: ResMut<StartGame>,
    game_state: Res<State<GameState>>,
    
){
    match game_state.get(){
        GameState::Result => {
            *writer.text(*score_text, 0) = format!("しゅうりょう！あなたのスコアは : {}",start_info.score);
            *writer.text(*left_temperature_text, 0) = format!("");
            *writer.text(*right_temperature_text, 0) = format!("エンターをおしてもどる");
        }
        GameState::Menu => {
            *writer.text(*score_text, 0) = format!("スペースキーをおしてスタート");
            *writer.text(*right_temperature_text, 0) = format!("");
        }
        _ => {}
    }
    
    

}

