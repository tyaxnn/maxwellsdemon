pub mod key_input;


use bevy::prelude::*;

use key_input::{
    exit_on_esc,
    remove_gap,
};

pub struct InputHandlePlugin;

impl Plugin for InputHandlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            exit_on_esc,
            remove_gap
        ));
    }
}