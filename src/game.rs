use bevy::prelude::*;
use crate::camera::MainCamera;
use crate::player::*;
use crate::platforms::{Platform, BrownPlatform};

#[derive(Component)]
pub struct Background {
    pub index: i16,
}

#[derive(Component)]
pub struct GameOverStruct;

#[derive(Component)]
pub struct PauseText;

#[derive(Component)]
pub struct StartText;

#[derive(Resource)]
pub struct Paused(pub bool);

#[derive(Resource, Default)]
pub struct CountStruct {
    pub count: u16,
}

#[derive(Component)]
pub struct CountText;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    NotStarted,
    InProcessGame,
    GameOver,
}

pub fn background_for_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("images/backgrounds/background.png");

    for i in -1..=1 {
        commands.spawn((
            Sprite {
                image: texture.clone(),
                ..default()
            },
            Transform::from_xyz(0.0, i as f32 * 850.0, -1.0),
            Background { index: i },
        ));
    }
}

pub fn background_follow_camera(
    camera_q: Query<&Transform, (With<MainCamera>, Without<Background>, Without<Player>)>,
    mut background_q: Query<(&mut Transform, &mut Background), (Without<MainCamera>, Without<Player>)>,
) {
    let Ok(camera_transform) = camera_q.get_single() else { return };
    let camera_y = camera_transform.translation.y;

    for (mut background_transform, mut background) in background_q.iter_mut() {
        let background_y = background_transform.translation.y;
        if camera_y - background_y > 850.0 {
            background_transform.translation.y += 3.0 * 850.0;
            background.index += 3;
        } else if background_y - camera_y > 850.0 {
            background_transform.translation.y -= 3.0 * 850.0;
            background.index -= 3;
        }
    }
}

pub fn pause(
    mut paused: ResMut<Paused>,
    key_code: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pause_query: Query<Entity, With<PauseText>>
) {
    if key_code.just_pressed(KeyCode::KeyP) {
        paused.0 = !paused.0;


        if paused.0 {
            commands.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                PauseText
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Пауза"),
                    TextFont {
                        font: asset_server.load("fonts/Arsenal-Regular.ttf"),
                        font_size: 60.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                ));
            });
            commands.spawn((
                AudioPlayer::new(asset_server.load("music/pause.mp3")),
                PlaybackSettings::ONCE,
            ));
        } else {
            for entity in pause_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn game_over(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<&Transform, With<Player>>,
    state: Res<State<GameState>>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Player>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if *state.get() == GameState::InProcessGame {
        if let Ok(player_transform) = query.get_single() {
            if let Ok(camera_transform) = camera_query.get_single() {
                if player_transform.translation.y < camera_transform.translation.y - 500.0 {
                    next_state.set(GameState::GameOver);
                    commands.spawn((
                        AudioPlayer::new(asset_server.load("music/game_over.mp3")),
                        PlaybackSettings::ONCE,
                    ));
                }
            }
        }
    }
}

pub fn show_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<GameState>>,
    game_over_query: Query<Entity, With<GameOverStruct>>,
    count: Res<CountStruct>,
) {
    if *state.get() == GameState::GameOver && game_over_query.is_empty() {
        let high_score = std::fs::read_to_string("high_score.txt")
            .unwrap_or_else(|_| "0".to_string())
            .parse::<u16>()
            .unwrap_or(0);

        let is_new_record = count.count > high_score;

        if is_new_record {
            let _ = std::fs::write("high_score.txt", count.count.to_string());
        }

        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            GameOverStruct
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Гра програна!"),
                TextFont {
                    font: asset_server.load("fonts/Arsenal-Regular.ttf"),
                    font_size: 50.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            // Поточний рахунок
            parent.spawn((
                Text::new(format!("Ваш рахунок: {}", count.count)),
                TextFont {
                    font: asset_server.load("fonts/Mariupol-Bold.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            if is_new_record {
                parent.spawn((
                    Text::new("Новий рекорд!"),
                    TextFont {
                        font: asset_server.load("fonts/Mariupol-Medium.ttf"),
                        font_size: 45.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                ));
            } else {
                parent.spawn((
                    Text::new(format!("Рекорд: {}", high_score)),
                    TextFont {
                        font: asset_server.load("fonts/Mariupol-Regular.ttf"),
                        font_size: 35.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                ));
            }

            parent.spawn((
                Text::new("Натисніть R для перезапуску"),
                TextFont {
                    font: asset_server.load("fonts/times.ttf"),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        });
    }
}

pub fn show_start_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<GameState>>,
    start_query: Query<Entity, With<StartText>>
) {
    if *state.get() == GameState::NotStarted && start_query.is_empty() {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            StartText
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Натисніть M для початку гри"),
                TextFont {
                    font: asset_server.load("fonts/Arsenal-Regular.ttf"),
                    font_size: 50.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        });
    }
}

pub fn clean_start_screen(
    mut commands: Commands,
    start_query: Query<Entity, With<StartText>>,
) {
    for entity in start_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn clean_on_restart(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    platform_query: Query<Entity, With<Platform>>,
    brown_platform_query: Query<Entity, With<BrownPlatform>>,
    game_over_query: Query<Entity, With<GameOverStruct>>,
    start_query: Query<Entity, With<StartText>>,
    pause_query: Query<Entity, With<PauseText>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Background>)>,
    mut background_query: Query<(&mut Transform, &Background), (With<Background>, Without<MainCamera>)>,
    mut count: ResMut<CountStruct>,
) {
    for entity in player_query.iter() {
        let _ = commands.get_entity(entity).map(|mut entity_commands|
            entity_commands.try_despawn()
        );
    }

    for entity in platform_query.iter() {
        let _ = commands.get_entity(entity).map(|mut entity_commands|
            entity_commands.try_despawn()
        );
    }

    for entity in brown_platform_query.iter() {
        let _ = commands.get_entity(entity).map(|mut entity_commands|
            entity_commands.try_despawn()
        );
    }

    for entity in game_over_query.iter() {
        let _ = commands.get_entity(entity).map(|entity_commands|
            entity_commands.try_despawn_recursive()
        );
    }

    for entity in start_query.iter() {
        let _ = commands.get_entity(entity).map(|entity_commands|
            entity_commands.try_despawn_recursive()
        );
    }

    for entity in pause_query.iter() {
        let _ = commands.get_entity(entity).map(|entity_commands|
            entity_commands.try_despawn_recursive()
        );
    }

    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        camera_transform.translation.y = 0.0;
    }

    for (mut bg_transform, bg) in background_query.iter_mut() {
        bg_transform.translation.y = bg.index as f32 * 850.0;
    }

    count.count = 0;
}

pub fn restart(
    mut next_state: ResMut<NextState<GameState>>,
    key_code: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>
) {
    if key_code.just_pressed(KeyCode::KeyR) && *state.get() == GameState::GameOver {
        next_state.set(GameState::NotStarted);
    }
}

pub fn start_game(
    mut next_state: ResMut<NextState<GameState>>,
    key_code: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>
) {
    if key_code.just_pressed(KeyCode::KeyM) && *state.get() == GameState::NotStarted {
        next_state.set(GameState::InProcessGame);
    }
}

pub fn not_paused(paused: Res<Paused>) -> bool {
    !paused.0
}

pub fn setup_count(mut commands: Commands, count: Res<CountStruct>,
asset_server: Res<AssetServer>) {
    commands.spawn((
        CountText,
        Text::new(format!("Рахунок: {}", count.count)),
        TextFont {
            font: asset_server.load("fonts/Arsenal-Regular.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::BLACK),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }
    ));
}

pub fn update_count(
    mut count: ResMut<CountStruct>,
    player_q: Query<&Transform, With<Player>>,
    platform_q: Query<&Transform, (With<Platform>, Without<BrownPlatform>)>,
    mut count_text_query: Query<&mut Text, With<CountText>>,
) {
    if let Ok(player_transform) = player_q.get_single() {
        for platform_transform in platform_q.iter() {
            let collision = player_transform.translation.y > platform_transform.translation.y
                && player_transform.translation.y - 50.0 < platform_transform.translation.y
                && (player_transform.translation.x - platform_transform.translation.x).abs() < 50.0;

            if collision {
                count.count += 1;
                for mut text in count_text_query.iter_mut() {
                    **text = format!("Рахунок: {}", count.count);
                }
            }
        }
    }
}
