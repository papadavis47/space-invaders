use bevy::{
    prelude::*,
    sprite::Sprite,
    window::{Window, WindowPlugin},
};

const WINDOW_WIDTH: f32 = 900.0;
const WINDOW_HEIGHT: f32 = 700.0;
const PLAYER_SPEED: f32 = 500.0;
const BULLET_SPEED: f32 = 650.0;
const ENEMY_BULLET_SPEED: f32 = 300.0;
const PLAYER_SIZE: Vec2 = Vec2::new(52.0, 24.0);
const INVADER_SIZE: Vec2 = Vec2::new(36.0, 24.0);
const BULLET_SIZE: Vec2 = Vec2::new(5.0, 16.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.015, 0.02, 0.06)))
        .insert_resource(GameStats::default())
        .insert_resource(EnemyFormation {
            direction: 1.0,
            speed: 55.0,
            drop_distance: 24.0,
        })
        .insert_resource(EnemyFireTimer(Timer::from_seconds(
            0.9,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space Invaders".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(
            Update,
            (
                move_player,
                player_shoot,
                move_bullets,
                move_invaders,
                enemy_shoot,
                detect_collisions,
                detect_player_hits,
                check_game_over,
                update_hud,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_game)
        .add_systems(OnEnter(GameState::GameOver), setup_game_over)
        .add_systems(Update, restart_game.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource)]
struct GameStats {
    score: u32,
    lives: u8,
    won: bool,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            score: 0,
            lives: 3,
            won: false,
        }
    }
}

#[derive(Resource)]
struct EnemyFormation {
    direction: f32,
    speed: f32,
    drop_distance: f32,
}

#[derive(Resource, Deref, DerefMut)]
struct EnemyFireTimer(Timer);

#[derive(Component)]
struct GameEntity;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Invader;

#[derive(Component)]
struct PlayerBullet;

#[derive(Component)]
struct EnemyBullet;

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct GameOverScreen;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_game(mut commands: Commands, mut stats: ResMut<GameStats>) {
    stats.score = 0;
    stats.lives = 3;
    stats.won = false;

    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.9, 0.35), PLAYER_SIZE),
        Transform::from_xyz(0.0, -290.0, 0.0),
        Player,
        GameEntity,
    ));

    let columns = 10;
    let rows = 5;
    let spacing = Vec2::new(58.0, 44.0);
    let start_x = -(columns as f32 - 1.0) * spacing.x / 2.0;

    for row in 0..rows {
        for column in 0..columns {
            let color = match row {
                0 => Color::srgb(0.95, 0.3, 0.65),
                1 | 2 => Color::srgb(0.95, 0.7, 0.2),
                _ => Color::srgb(0.35, 0.8, 1.0),
            };

            commands.spawn((
                Sprite::from_color(color, INVADER_SIZE),
                Transform::from_xyz(
                    start_x + column as f32 * spacing.x,
                    235.0 - row as f32 * spacing.y,
                    0.0,
                ),
                Invader,
                GameEntity,
            ));
        }
    }

    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 26.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(20.0),
            ..default()
        },
        Hud,
        GameEntity,
    ));
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let mut direction = 0.0;
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction += 1.0;
    }

    player.translation.x += direction * PLAYER_SPEED * time.delta_secs();
    let limit = WINDOW_WIDTH / 2.0 - PLAYER_SIZE.x / 2.0;
    player.translation.x = player.translation.x.clamp(-limit, limit);
}

fn player_shoot(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Single<&Transform, With<Player>>,
    existing_bullets: Query<(), With<PlayerBullet>>,
) {
    if keyboard.just_pressed(KeyCode::Space) && existing_bullets.is_empty() {
        commands.spawn((
            Sprite::from_color(Color::WHITE, BULLET_SIZE),
            Transform::from_xyz(
                player.translation.x,
                player.translation.y + PLAYER_SIZE.y,
                0.0,
            ),
            PlayerBullet,
            GameEntity,
        ));
    }
}

fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut player_bullets: Query<(Entity, &mut Transform), With<PlayerBullet>>,
    mut enemy_bullets: Query<(Entity, &mut Transform), (With<EnemyBullet>, Without<PlayerBullet>)>,
) {
    for (entity, mut transform) in &mut player_bullets {
        transform.translation.y += BULLET_SPEED * time.delta_secs();
        if transform.translation.y > WINDOW_HEIGHT / 2.0 {
            commands.entity(entity).despawn();
        }
    }

    for (entity, mut transform) in &mut enemy_bullets {
        transform.translation.y -= ENEMY_BULLET_SPEED * time.delta_secs();
        if transform.translation.y < -WINDOW_HEIGHT / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn move_invaders(
    time: Res<Time>,
    mut formation: ResMut<EnemyFormation>,
    mut invaders: Query<&mut Transform, With<Invader>>,
) {
    let edge = WINDOW_WIDTH / 2.0 - INVADER_SIZE.x;
    let should_turn = invaders.iter().any(|transform| {
        (formation.direction > 0.0 && transform.translation.x >= edge)
            || (formation.direction < 0.0 && transform.translation.x <= -edge)
    });

    if should_turn {
        formation.direction *= -1.0;
        formation.speed += 4.0;
        for mut transform in &mut invaders {
            transform.translation.y -= formation.drop_distance;
        }
    } else {
        let movement = formation.direction * formation.speed * time.delta_secs();
        for mut transform in &mut invaders {
            transform.translation.x += movement;
        }
    }
}

fn enemy_shoot(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemyFireTimer>,
    invaders: Query<&Transform, With<Invader>>,
) {
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    // Pick a pseudo-random shooter without needing an extra RNG dependency.
    let invader_count = invaders.iter().count();
    if invader_count == 0 {
        return;
    }
    let index = (time.elapsed_secs_f64() * 1000.0) as usize % invader_count;
    if let Some(transform) = invaders.iter().nth(index) {
        commands.spawn((
            Sprite::from_color(Color::srgb(1.0, 0.25, 0.2), BULLET_SIZE),
            Transform::from_xyz(
                transform.translation.x,
                transform.translation.y - INVADER_SIZE.y,
                0.0,
            ),
            EnemyBullet,
            GameEntity,
        ));
    }
}

fn detect_collisions(
    mut commands: Commands,
    mut stats: ResMut<GameStats>,
    bullets: Query<(Entity, &Transform), With<PlayerBullet>>,
    invaders: Query<(Entity, &Transform), With<Invader>>,
) {
    for (bullet_entity, bullet_transform) in &bullets {
        for (invader_entity, invader_transform) in &invaders {
            if overlaps(
                bullet_transform.translation.truncate(),
                BULLET_SIZE,
                invader_transform.translation.truncate(),
                INVADER_SIZE,
            ) {
                commands.entity(bullet_entity).despawn();
                commands.entity(invader_entity).despawn();
                stats.score += 10;
                break;
            }
        }
    }
}

fn detect_player_hits(
    mut commands: Commands,
    mut stats: ResMut<GameStats>,
    player: Single<&Transform, With<Player>>,
    bullets: Query<(Entity, &Transform), With<EnemyBullet>>,
) {
    for (bullet_entity, bullet_transform) in &bullets {
        if overlaps(
            bullet_transform.translation.truncate(),
            BULLET_SIZE,
            player.translation.truncate(),
            PLAYER_SIZE,
        ) {
            commands.entity(bullet_entity).despawn();
            stats.lives = stats.lives.saturating_sub(1);
        }
    }
}

fn check_game_over(
    invaders: Query<&Transform, With<Invader>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_stats: ResMut<GameStats>,
) {
    if invaders.is_empty() {
        game_stats.won = true;
        next_state.set(GameState::GameOver);
    } else if game_stats.lives == 0
        || invaders
            .iter()
            .any(|transform| transform.translation.y <= -255.0)
    {
        game_stats.won = false;
        next_state.set(GameState::GameOver);
    }
}

fn update_hud(stats: Res<GameStats>, mut hud: Single<&mut Text, With<Hud>>) {
    if stats.is_changed() {
        **hud = format!("SCORE  {:04}       LIVES  {}", stats.score, stats.lives).into();
    }
}

fn cleanup_game(mut commands: Commands, entities: Query<Entity, With<GameEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn setup_game_over(mut commands: Commands, stats: Res<GameStats>) {
    let message = if stats.won { "YOU WIN!" } else { "GAME OVER" };
    commands.spawn((
        Text::new(format!(
            "{message}\n\nScore: {}\n\nPress ENTER to play again",
            stats.score
        )),
        TextFont {
            font_size: 42.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            top: Val::Percent(32.0),
            ..default()
        },
        GameOverScreen,
    ));
}

fn restart_game(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
}

fn cleanup_game_over(
    mut commands: Commands,
    screens: Query<Entity, With<GameOverScreen>>,
    mut formation: ResMut<EnemyFormation>,
) {
    for entity in &screens {
        commands.entity(entity).despawn();
    }
    formation.direction = 1.0;
    formation.speed = 55.0;
}

fn overlaps(a_position: Vec2, a_size: Vec2, b_position: Vec2, b_size: Vec2) -> bool {
    let distance = (a_position - b_position).abs();
    distance.x < (a_size.x + b_size.x) / 2.0 && distance.y < (a_size.y + b_size.y) / 2.0
}
