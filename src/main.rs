use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
    window::WindowMode,
};

// 1280x720 = HD ready
const VIEW_WIDTH: f32 = 1280.0;
const VIEW_HEIGHT: f32 = 9.0 * VIEW_WIDTH / 16.0;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;

// TODO make this changable later
const PADDLE_SIZE: Vec3 = Vec3::new(20.0, 120.0, 0.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

const BALL_SPEED: f32 = 400.0;
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(640.0, 350.0, 1.0);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

const LEFT_PADDLE_A_X: f32 = 50.0;
const RIGHT_PADDLE_A_X: f32 = VIEW_WIDTH - 50.0;
const COMMON_PADDLE_Y: f32 = (VIEW_HEIGHT / 2.0) - 60.0;

const BANTE_THICKNESS: f32 = 10.0;
// TODO make this changable later
const BANTE_SIZE: Vec3 = Vec3::new(VIEW_WIDTH, BANTE_THICKNESS, 0.0);
const BANTE_COLOR: Color = Color::rgb(0.3, 0.7, 0.7);
const BOTTOM_BANTE: f32 = 0.0;
const TOP_BANTE: f32 = VIEW_HEIGHT;

const BOTTOM_BOUND: f32 =
    BOTTOM_BANTE + BANTE_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0 + PADDLE_PADDING;
const TOP_BOUND: f32 = TOP_BANTE - BANTE_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0 - PADDLE_PADDING;

/*

// x coordinates
const LEFT_WALL: f32 = 0.0;
const RIGHT_WALL: f32 = VIEW_WIDTH;
 */

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Bante;

// It seems like this is being used finding objects that
#[derive(Component)]
struct Collider;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn main() {
    println!("Bevy - pong 0.1.0");
    // Systems run in parallel
    App::new()
        .insert_resource(WindowDescriptor {
            width: VIEW_WIDTH,
            height: VIEW_HEIGHT,
            //mode: WindowMode::Fullscreen,
            title: "PONG 0.1.0".to_string(),
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_pong_game)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
                .with_system(move_paddle)
                .with_system(apply_velocity),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_pong_game(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // The screen randomly does not show the paddle, re-running a couple of times
    //   and it will turn up.

    println!(
        "DDD LEFT_PADDLE_A_X: {}, COMMON_PADDLE_Y: {}",
        LEFT_PADDLE_A_X, COMMON_PADDLE_Y
    );

    // left paddle
    commands.spawn().insert(Paddle).insert_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(LEFT_PADDLE_A_X, COMMON_PADDLE_Y, 0.0),
            scale: PADDLE_SIZE,
            ..default()
        },
        sprite: Sprite {
            color: PADDLE_COLOR,
            ..default()
        },
        ..default()
    });

    // left paddle
    commands.spawn().insert(Paddle).insert_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(RIGHT_PADDLE_A_X, COMMON_PADDLE_Y, 0.0),
            scale: PADDLE_SIZE,
            ..default()
        },
        sprite: Sprite {
            color: PADDLE_COLOR,
            ..default()
        },
        ..default()
    });
    /*
    From : ~/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_ecs-0.8.1/src/system/commands/mod.rs
    spawn - spawn an entity. (I assume that you wont get the specific ID out of this).
      * returns EntityCommands
    insert - inser component (on new or existing entities)
      * Adds a single [`Component`] to the entity.
      * returns self
    insert_bundle -

    SpriteBundle -
    Transform -

     */
    // Ball
    commands
        .spawn()
        .insert(Ball)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: BALL_SIZE,
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED));

    // bottom bante
    commands
        .spawn()
        .insert(Bante)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(VIEW_WIDTH / 2.0, BANTE_THICKNESS / 2.0, 0.0),
                scale: BANTE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: BANTE_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Collider);
    // top bante
    commands
        .spawn()
        .insert(Bante)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(VIEW_WIDTH / 2.0,  VIEW_HEIGHT - BANTE_THICKNESS / 2.0, 0.0),
                scale: BANTE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: BANTE_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Collider);
}

fn spawn_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::new_with_far(1000.0);
    // See: https://www.youtube.com/watch?v=WnUzWuaMzuM
    camera_bundle.projection.scaling_mode = ScalingMode::None;
    camera_bundle.projection.top = VIEW_HEIGHT;
    camera_bundle.projection.bottom = 0.0;
    camera_bundle.projection.left = 0.0;
    camera_bundle.projection.right = VIEW_WIDTH;
    commands.spawn_bundle(camera_bundle);
}

fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
) {
    for mut paddle_transform in paddle_query.iter_mut() {
        // let mut paddle_transform = paddle_query.single_mut();
        let mut paddle_direction = 0.0;

        if paddle_transform.translation.x == LEFT_PADDLE_A_X {
            if keyboard_input.pressed(KeyCode::W) {
                paddle_direction += 1.0;
            }
            if keyboard_input.pressed(KeyCode::S) {
                paddle_direction -= 1.0;
            }
        }

        if paddle_transform.translation.x == RIGHT_PADDLE_A_X {
            if keyboard_input.pressed(KeyCode::Up) {
                paddle_direction += 1.0;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                paddle_direction -= 1.0;
            }
        }
        // Calculate the new horizontal paddle position based on player input
        let new_paddle_position =
            paddle_transform.translation.y + paddle_direction * PADDLE_SPEED * TIME_STEP;

        // Update the paddle position,
        // making sure it doesn't cause the paddle to leave the arena

        paddle_transform.translation.y = new_paddle_position.clamp(BOTTOM_BOUND, TOP_BOUND);
    }
}

// TODO put a thin wall at the bottom and the topc
// TODO where do I put the code for putting  a spin on it? When it hit the paddle.
fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

// TODO use event to tell whether or not the ball is in play.
/* clear event when ball disapear */

/*
ball_query - is not a parm, it just get done at the start TODO why?
 */
fn check_for_collisions(
    mut _commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    paddle_query: Query<(&Transform), With<Paddle>>,
    bante_query: Query<(Entity, &Transform), With<Bante>>,
) {
    // TODO what is transform?
    // TODO what is translation?(is it the position of the object?)
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    //let ball_size = ball_transform.scale.truncate();
    //let collision = collide(ball_transform.translation, ball_size, b_pos, b_size)
    let ball_x = ball_transform.translation[0];
    // println!("DDD ball pos {}", ball_transform.translation);
    // TODO create the "Walls" as entities, then if Collision::Top or bottom, then bounce
    // TODO otherwise the ball is out of bounds and awards points etc.
    if ball_x > VIEW_WIDTH {
        println!(
            "DDD ball exit stage right: pos {}",
            ball_transform.translation
        );
        ball_velocity.x = 0.0;
        ball_velocity.y = 0.0;
        // TODO give points to left player
        // TODO now move the ball left, when the ball is started again (in the center)
    }
    let ball_size = ball_transform.scale.truncate();
    // check collision with paddle
    // TODO fix the distance so the ball 'hits' the paddle)
    for transform in &paddle_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            // TODO do I need this??? collision_events.send_default();

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
    // check collision with walls
    for (_collider_entity, transform) in &bante_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            // TODO do I need this??? collision_events.send_default();

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}
