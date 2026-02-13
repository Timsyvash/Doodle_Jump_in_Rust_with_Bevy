use bevy::prelude::*;
use crate::platforms::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct VelocityInY {
    pub y: f32,
}

pub fn gravity_player(
    mut transform_query: Query<(&mut Transform, &mut VelocityInY), With<Player>>,
time: Res<Time>) {
    let gravity = -300.0;

    for (mut t, mut vel) in &mut transform_query {
        vel.y += gravity * time.delta_secs();
        t.translation.y += vel.y * time.delta_secs();
    }
}

pub fn load_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load("images/players/player1.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 200.0, 0.5),
        Player,
        VelocityInY { y: 0.0 },
    ));
}

pub fn player_control_in_x(key_code: Res<ButtonInput<KeyCode>>,
            mut query: Query<(&mut Transform, &mut Sprite), With<Player>>,
) {
    for (mut t, mut tex) in query.iter_mut() {
        if key_code.just_pressed(KeyCode::ArrowLeft) || key_code.just_pressed(KeyCode::KeyA) {
            t.translation.x -= 30.0;
            tex.flip_x = true;
        }
        if key_code.just_pressed(KeyCode::ArrowRight) || key_code.just_pressed(KeyCode::KeyD) {
            t.translation.x += 30.0;
            tex.flip_x = false;
        }
    }
}

pub fn collision_player_with_platforms (
    mut player_q: Query<(&Transform, &mut VelocityInY), With<Player>>,
    platform_q: Query<&Transform, (With<Platform>, Without<BrownPlatform>)>,
    mut commands: Commands, asset_server: Res<AssetServer>
) {
    for (player, mut vel_y) in player_q.iter_mut() {
        for platform in platform_q.iter() {
            let collision = player.translation.y > platform.translation.y
                && player.translation.y - 50.0 < platform.translation.y
                && (player.translation.x - platform.translation.x).abs() < 50.0;

            if collision && vel_y.y <= 0.0 {
                vel_y.y = 800.0;
                commands.spawn((
                    AudioPlayer::new(asset_server.load("music/jump.mp3")),
                    PlaybackSettings::ONCE,
                ));
            }
        }
    }
}

pub fn borders(
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut t in query.iter_mut() {
        if t.translation.x > 210.0 {
            t.translation.x = 210.0;
        }
        if t.translation.x < -210.0 {
            t.translation.x = -210.0;
        }
    }
}
 