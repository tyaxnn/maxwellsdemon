pub mod key_input;


use bevy::prelude::*;

use key_input::{
    exit_on_esc,
    remove_gap,
    akuma_moves,
    change_state_to_game,
    change_state_to_menu,
};

use crate::GameState;

pub struct InputHandlePlugin;

impl Plugin for InputHandlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            exit_on_esc,
            remove_gap.run_if(in_state(GameState::Game)),
            akuma_moves.run_if(in_state(GameState::Game)),
            change_state_to_game.run_if(in_state(GameState::Menu)),
            change_state_to_menu.run_if(in_state(GameState::Result)),
        ));
    }
}