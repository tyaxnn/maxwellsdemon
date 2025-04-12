use bevy::prelude::*;

pub mod spawn_wall;
pub mod spawn_camera;
pub mod spawn_ball;

use spawn_wall::spawn_walls;
use spawn_camera::spawn_camera;
use spawn_ball::spawn_balls;

static FONT_PATH: &str = "fonts/consolas.ttf";

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
        .add_systems(PostStartup, spawn_balls);
    }
}

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Akuma;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    spawn_walls(&mut commands);
    spawn_camera(&mut commands);

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

    let texture_handle = asset_server.load("pic/character_akuma.png"); // assets/images/your_image.png

    commands.spawn((Sprite{
        image : texture_handle,
        ..default()
    },
    Transform::from_xyz(0.0, 240.0, 100.0).with_scale(Vec3::new(0.5,0.5,0.5)),
    Akuma,
    )
);

}
