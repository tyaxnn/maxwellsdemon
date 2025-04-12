use bevy::prelude::*;

// x coordinates
pub const LEFT_WALL: f32 = -500.;
pub const RIGHT_WALL: f32 = 500.;
// y coordinates
pub const BOTTOM_WALL: f32 = -300.;
pub const TOP_WALL: f32 = 300.;

const GAP: f32 = 25.;

const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const WALL_COLOR_GAP: Color = Color::srgb(0.1, 1.0, 0.1);

pub const WALL_THICKNESS: f32 = 10.0;
pub const CENTER_THICHNESS: f32 = 300.0;
pub const GAP_THICKNESS: f32 = 5.0;

#[derive(Bundle)]

struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite: Sprite,
    transform: Transform,
    collider: Collider,
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite: Sprite::from_color(WALL_COLOR, Vec2::ONE),
            transform: Transform {
                // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                // This is used to determine the order of our sprites
                translation: location.position().extend(0.0),
                // The z-scale of 2D objects must always be 1.0,
                // or their ordering will be affected in surprising ways.
                // See https://github.com/bevyengine/bevy/issues/4149
                scale: location.size().extend(1.0),
                ..default()
            },
            collider: Collider,
        }
    }

    fn new_gap(location : WallLocation) -> WallBundle {
        WallBundle {
            sprite: Sprite::from_color(WALL_COLOR_GAP, Vec2::ONE),
            transform: Transform {
                // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                // This is used to determine the order of our sprites
                translation: location.position().extend(0.0),
                // The z-scale of 2D objects must always be 1.0,
                // or their ordering will be affected in surprising ways.
                // See https://github.com/bevyengine/bevy/issues/4149
                scale: location.size().extend(1.0),
                ..default()
            },
            collider: Collider,
        }
    }

}

#[derive(Component)]
pub struct Collider;


enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
    CenterTop,
    CenterBottom,
    Gap,
}




impl WallLocation {
    /// Location of the *center* of the wall, used in `transform.translation()`
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
            WallLocation::CenterTop => Vec2::new(0.,TOP_WALL / 2. + GAP /4.),
            WallLocation::CenterBottom => Vec2::new(0.,BOTTOM_WALL / 2. - GAP /4.),
            WallLocation::Gap => Vec2::new(0., 0.),
        }
    }
    /// (x, y) dimensions of the wall, used in `transform.scale()`
    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
            WallLocation::CenterBottom | WallLocation::CenterTop => {
                Vec2::new(CENTER_THICHNESS, TOP_WALL - GAP/2.)
            }
            WallLocation::Gap => Vec2::new(GAP_THICKNESS, GAP)
        }
    }
    
}

#[derive(Component)]
pub struct RemovableWall(pub bool);

pub fn spawn_walls(
    commands: &mut Commands,    
){
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::CenterBottom));
    commands.spawn(WallBundle::new(WallLocation::CenterTop));
    commands.spawn((
        WallBundle::new_gap(WallLocation::Gap),
        RemovableWall(false)
    ));
}