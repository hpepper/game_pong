use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
    time::Stopwatch,
    //window::WindowMode,
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
const LEFT_PADDLE_COLOR: Color = Color::BLUE;
const RIGHT_PADDLE_COLOR: Color = Color::YELLOW_GREEN;

const BALL_SPEED: f32 = 400.0;
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(640.0, 350.0, 1.0);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

// five seconds seems to be too short, when the speed increase is 0.1
const INCREASE_BALL_VELOSITY_AFTER_TIME_WITHOUT_SCORE: f32 = 10.0;
const DELTA_INCREASE_IN_BALL_SPEED: f32 = 0.05 * BALL_SPEED;

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

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_TOP_PADDING: Val = Val::Px(5.0);
const SCOREBOARD_TEXT_LEFT_PADDING: Val = Val::Px(VIEW_WIDTH / 2.0);

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

// A resource to use for increasing speed when x seconds have passed since last score.
struct TimeSinceLastScore {
    stopwatch_timer: Stopwatch,
}

// Currently used for playing sound when the ball hits something
#[derive(Default)]
struct CollisionEvent;
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

struct Scoreboard {
    left_player_score: usize,
    right_player_score: usize,
}

struct CollisionSound(Handle<AudioSource>);

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
        .insert_resource(Scoreboard {
            left_player_score: 0,
            right_player_score: 0,
        })
        .insert_resource(TimeSinceLastScore {
            stopwatch_timer: Stopwatch::new(),
        })
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_pong_game)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
                .with_system(move_paddle)
                .with_system(apply_velocity)
                .with_system(play_collision_sound.after(check_for_collisions)),
        )
        .add_system(update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_pong_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // The screen randomly does not show the paddle, re-running a couple of times
    //   and it will turn up.

    println!(
        "DDD LEFT_PADDLE_A_X: {}, COMMON_PADDLE_Y: {}",
        LEFT_PADDLE_A_X, COMMON_PADDLE_Y
    );

    // Sound
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    // left paddle
    commands.spawn().insert(Paddle).insert_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(LEFT_PADDLE_A_X, COMMON_PADDLE_Y, 0.0),
            scale: PADDLE_SIZE,
            ..default()
        },
        sprite: Sprite {
            color: LEFT_PADDLE_COLOR,
            ..default()
        },
        ..default()
    });

    // right paddle
    commands.spawn().insert(Paddle).insert_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(RIGHT_PADDLE_A_X, COMMON_PADDLE_Y, 0.0),
            scale: PADDLE_SIZE,
            ..default()
        },
        sprite: Sprite {
            color: RIGHT_PADDLE_COLOR,
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

    // Scoreboard
    commands.spawn_bundle(
        TextBundle::from_sections([
            TextSection::new(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: LEFT_PADDLE_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: SCOREBOARD_FONT_SIZE,
                color: RIGHT_PADDLE_COLOR,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: SCOREBOARD_TEXT_TOP_PADDING,
                left: SCOREBOARD_TEXT_LEFT_PADDING,
                ..default()
            },
            ..default()
        }),
    );

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
                translation: Vec3::new(VIEW_WIDTH / 2.0, VIEW_HEIGHT - BANTE_THICKNESS / 2.0, 0.0),
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
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Velocity, &mut Transform), With<Ball>>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    bante_query: Query<(Entity, &Transform), (With<Bante>, Without<Ball>)>,
    mut score_time_elapsed: ResMut<TimeSinceLastScore>,
    time: Res<Time>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    score_time_elapsed.stopwatch_timer.tick(time.delta());
    // TODO what is transform?
    // TODO what is translation?(is it the position of the object?)
    let (mut ball_velocity, mut ball_transform) = ball_query.single_mut();
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
        // give points to left player
        scoreboard.left_player_score += 1;
        // now move the ball left, when the ball is started again (in the center)
        ball_velocity.x = -ball_velocity.x;
        // tranport the ball to the center
        ball_transform.translation.x = VIEW_WIDTH / 2.0;
        score_time_elapsed.stopwatch_timer.reset();
    }
    if ball_x <= 0.0 {
        // TODO play a sound when the ball exit the field
        println!(
            "DDD ball exit stage left: pos {}",
            ball_transform.translation
        );
        scoreboard.right_player_score += 1;
        ball_velocity.x = -ball_velocity.x;
        //ball_velocity.y = 0.0;
        // tranport the ball to the center
        ball_transform.translation.x = VIEW_WIDTH / 2.0;
        score_time_elapsed.stopwatch_timer.reset();
    }

    if score_time_elapsed.stopwatch_timer.elapsed_secs()
        > INCREASE_BALL_VELOSITY_AFTER_TIME_WITHOUT_SCORE
    {
        // TODO speed up, set a timer, that gets reset, ever time there is a score
        //  if the timer is hit then increase the ball velosity.
        //  later have a directional velocity, so it has one speed going in one direction and onther velocity going in the other direction
        // TODO at some point, maybe have a speed limit so the paddles have a chance of following along?
        // Maybe just add a constant number instead of this increasing percentage???
        if ball_velocity.x > 0.0 {
            ball_velocity.x += DELTA_INCREASE_IN_BALL_SPEED;
        } else {
            ball_velocity.x -= DELTA_INCREASE_IN_BALL_SPEED;
        }
        println!(
            "DDD more than {} secs since last score, increase velosity by {} to {}",
            score_time_elapsed.stopwatch_timer.elapsed_secs(),
            DELTA_INCREASE_IN_BALL_SPEED,
            ball_velocity.x.abs()
        );
        score_time_elapsed.stopwatch_timer.reset();
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
            collision_events.send_default();

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
            collision_events.send_default();

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

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[0].value = scoreboard.left_player_score.to_string();
    text.sections[1].value = scoreboard.right_player_score.to_string();
}

fn play_collision_sound(
    collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        collision_events.clear();
        audio.play(sound.0.clone());
    }
}
