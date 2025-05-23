use bevy::{color::palettes::css::{BLACK, RED}, prelude::*};

use crate::scores::OldScores;

static FONT_PATH: &str = "fonts/CherryBombOne-Regular.ttf";

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct RemainingTimeText;

#[derive(Component)]
pub struct LeftTemperatureText;

#[derive(Component)]
pub struct RightTemperatureText;
#[derive(Component)]
pub struct DemonSerif;
#[derive(Component)]
pub struct Top10text;

pub fn spawn_score(
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
            left: Val::Auto, 
            top: Val::Percent(94.0),
            ..default()
        },

        TextLayout::new_with_justify(JustifyText::Center),

        ScoreText
    ));
}

pub fn spawn_left_temperature(
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
            left: Val::Percent(-30.0), 
            top: Val::Percent(94.0),
            ..default()
        },

        TextLayout::new_with_justify(JustifyText::Center),

        LeftTemperatureText
    ));
}

pub fn spawn_right_temperature(
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
            right: Val::Percent(-30.0), 
            top: Val::Percent(94.0),
            ..default()
        },

        TextLayout::new_with_justify(JustifyText::Center),

        RightTemperatureText
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
            left: Val::Percent(0.0), 
            top: Val::Percent(60.0),
            ..default()
        },

        TextLayout::new_with_justify(JustifyText::Center),

        RemainingTimeText,
        TextColor(Color::Srgba(RED))
    ));
}

pub fn spawn_demon_serif(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
){

    let font = asset_server.load(FONT_PATH);

    commands.spawn((
        Text::new("ハイスコアめざして\nがんばろう"),
        TextFont {
            font: font.clone(),
            font_size: 24.,
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
            left: Val::Percent(15.0), 
            top: Val::Percent(10.0),
            ..default()
        },

        TextLayout::new_with_justify(JustifyText::Center),

        DemonSerif,
        TextColor(Color::Srgba(BLACK)),
    ));
}

pub fn spawn_top10_text(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    old_scores: Res<OldScores>
){

    let font = asset_server.load(FONT_PATH);

    commands.spawn((
        Text::new(old_scores.display()),
        TextFont {
            font: font.clone(),
            font_size: 24.,
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
            left: Val::Percent(-25.0), 
            top: Val::Percent(20.0),
            ..default()
        },

        TextLayout::new_with_justify(JustifyText::Center),

        Top10text,
    ));
}