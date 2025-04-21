use bevy::prelude::*;

static FONT_PATH: &str = "fonts/consolas.ttf";

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct RemainingTimeText;

pub fn spawn_score(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
){

    let font = asset_server.load(FONT_PATH);

    commands.spawn((
        Text::new(""),
        TextFont {
            font: font.clone(),
            font_size: 14.,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            justify_items: JustifyItems::Center,
            justify_content: JustifyContent::Center, 
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            width: Val::Percent(100.0), 
            left: Val::Auto, 
            top: Val::Percent(95.0),
            ..default()
        },

        TextLayout::new_with_justify(JustifyText::Center),

        ScoreText
    ));
}

pub fn spawn_remaining_time(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
){

    let font = asset_server.load(FONT_PATH);

    commands.spawn((
        Text::new(""),
        TextFont {
            font: font.clone(),
            font_size: 34.,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            justify_items: JustifyItems::Center,
            justify_content: JustifyContent::Center, 
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            width: Val::Percent(100.0), 
            left: Val::Percent(10.0), 
            top: Val::Percent(10.0),
            ..default()
        },

        TextLayout::new_with_justify(JustifyText::Center),

        RemainingTimeText
    ));
}