use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    time::FixedTimestep,
    window::WindowMode
};

// 1280x720 ) HD ready
const VIEW_WIDTH: f32 = 1280.0;
const VIEW_HEIGHT: f32 = 9.0 * VIEW_WIDTH / 16.0;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;


const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;

const WALL_THICKNESS: f32 = 10.0;

/*

// x coordinates
const LEFT_WALL: f32 = 0.0;
const RIGHT_WALL: f32 = VIEW_WIDTH;
 */

// y coordinates
const BOTTOM_WALL: f32 = 0.0;
const TOP_WALL: f32 = VIEW_HEIGHT;


#[derive(Component)]
struct Paddle;

fn main() {
    println!("Bevy - pong 0.1.0");
    // Systems run in parallel
    App::new()
        .insert_resource(WindowDescriptor {
            width: VIEW_WIDTH,
            height: VIEW_HEIGHT,
            mode: WindowMode::Fullscreen,
            title: "PONG 0.1.0".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_pong_game)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_paddle)
        )
        .add_system(bevy::window::close_on_esc)
        .run();

}

// TODO make this changable later
const PADDLE_SIZE: Vec3 = Vec3::new(20.0, 120.0, 0.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

fn setup_pong_game(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // The screen randomly does not show the paddle, re-running a couple of times
    //   and it will turn up.
    let left_paddle_x: f32 = 50.0;

    let both_paddle_y: f32 = (VIEW_HEIGHT / 2.0) - 60.0;

    println!("DDD left_paddle_x: {}, both_paddle_y: {}", left_paddle_x,both_paddle_y );

    commands.spawn().insert(Paddle).insert_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(left_paddle_x, both_paddle_y, 0.0),
            scale: PADDLE_SIZE,
            ..default()
        },
        sprite: Sprite {
            color: PADDLE_COLOR,
            ..default()
        },
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::new_with_far(1000.0);
    // See: https://www.youtube.com/watch?v=WnUzWuaMzuM
    camera_bundle.projection.scaling_mode = ScalingMode::None;
    camera_bundle.projection.top = 720.0;
    camera_bundle.projection.bottom = 0.0;
    camera_bundle.projection.left = 0.0;
    camera_bundle.projection.right = 1280.0;
    commands.spawn_bundle(camera_bundle);
}


fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction += 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_position = paddle_transform.translation.y + direction * PADDLE_SPEED * TIME_STEP;

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0 + PADDLE_PADDING;
    let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0 - PADDLE_PADDING;

    paddle_transform.translation.y = new_paddle_position.clamp(bottom_bound, top_bound);
}
