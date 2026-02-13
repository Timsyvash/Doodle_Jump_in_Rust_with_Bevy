use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::camera::MainCamera;
use crate::player::*;

#[derive(Component)]
pub struct Platform;

#[derive(Component)]
pub struct BrownPlatform;

pub fn load_platforms(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut cur_y = -200.0;

    for _ in 1..=30 {
        let b = thread_rng().gen_bool(0.3);
        spawn_platform(&mut commands, &asset_server, cur_y, b);
        cur_y += thread_rng().gen_range(10.0..300.0);
    }
}

fn spawn_platform(commands: &mut Commands, asset_server: &Res<AssetServer>, y: f32,
brown: bool) {
    let x = thread_rng().gen_range(-240.0..240.0);

    let path = if brown {
        "images/platforms/platform_1.png".to_string()
    } else {
        format!(
            "images/platforms/platform_{}.png",
            thread_rng().gen_range(2..4)
        )
    };

    let mut entity = commands.spawn((
        Platform,
        Sprite {
            image: asset_server.load(path),
            ..default()
        },
        Transform::from_xyz(x, y, 0.5),
    ));

    if brown {
        entity.insert(BrownPlatform);
    }
}

pub fn limit_platforms(
    mut q: Query<&mut Transform, With<Platform>>,
) {
    for mut t in q.iter_mut() {
        if t.translation.x > 240.0 {
            t.translation.x = 240.0;
        }
        if t.translation.x < -240.0 {
            t.translation.x = -240.0;
        }
    }
}

pub fn distance_between_platforms(
    mut platform_q: Query<(Entity, &mut Transform), With<Platform>>,
) {
    let min_distance = 40.0;

    let platforms: Vec<(Entity, Transform)> = platform_q.iter_mut()
        .map(|(e, t)| (e, *t))
        .collect();

    for (entity_a, mut transform_a) in platform_q.iter_mut() {
        for (entity_b, transform_b) in &platforms {
            if entity_a != *entity_b {
                let distance = transform_a.translation.distance(transform_b.translation);
                if distance < min_distance {
                    let direction = (transform_a.translation - transform_b.translation).normalize();
                    transform_a.translation += direction * (min_distance - distance);
                }
            }
        }
    }
}

pub fn generate_platforms_for_move_camera(
    mut commands: Commands,
    platform_q: Query<&Transform, (With<Platform>, Without<MainCamera>, Without<Player>)>,
    camera_q: Query<&Transform, (With<MainCamera>, Without<Platform>, Without<Player>)>,
    asset_server: Res<AssetServer>,
) {
    let camera_transform = camera_q.single();
    let mut max_y = -600.0;

    for transform in platform_q.iter() {
        if transform.translation.y > max_y {
            max_y = transform.translation.y;
        }
    }

    if max_y < camera_transform.translation.y + 40.0 {
        let mut cur_y = max_y + thread_rng().gen_range(10.0..300.0);

        for _ in 1..30 {
            let b = thread_rng().gen_bool(0.3);
            spawn_platform(&mut commands, &asset_server, cur_y, b);
            cur_y += thread_rng().gen_range(10.0..300.0);
        }
    }
}

pub fn remove_platforms_below_camera(
    mut commands: Commands,
    platform_q: Query<(Entity, &Transform), (With<Platform>, Without<MainCamera>, Without<Player>)>,
    camera_q: Query<&Transform, (With<MainCamera>, Without<Platform>, Without<Player>)>,
) {
    let camera_transform = camera_q.single();

    for (entity, transform) in platform_q.iter() {
        if transform.translation.y < camera_transform.translation.y - 500.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn collision_player_with_brown_platforms (
    mut commands: Commands,
    mut player_q: Query<&Transform, With<Player>>,
    brown_platform_q: Query<(Entity, &Transform), With<BrownPlatform>>,
    asset_server: Res<AssetServer>
) {
    for player in player_q.iter_mut() {
        for (entity, platform) in brown_platform_q.iter() {
            let collision = player.translation.y > platform.translation.y
                && player.translation.y - 20.0 < platform.translation.y
                && (player.translation.x - platform.translation.x).abs() < 20.0;

            if collision {
                commands.spawn((
                    Sprite {
                        image: asset_server.load("images/platforms/platform_4.png"),
                        ..default()
                    },
                    Transform::from_xyz(platform.translation.x, platform.translation.y, 0.5),
                ));

                commands.entity(entity).despawn();
            }
        }
    }
}
 