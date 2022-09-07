use bevy::{prelude::*, render::camera::ScalingMode, window::WindowMode};

// 1280x720 ) HD ready
const VIEW_WIDTH: f32 = 1280.0;
const VIEW_HEIGHT: f32 = 9.0 * VIEW_WIDTH / 16.0;


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
