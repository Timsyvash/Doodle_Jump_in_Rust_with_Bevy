use bevy::prelude::*;
use crate::player::Player;

#[derive(Component)]
pub struct MainCamera;

pub fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        MainCamera,
    ));
}

pub fn move_camera(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else { return };

    if player_transform.translation.y > camera_transform.translation.y {
        camera_transform.translation.y = player_transform.translation.y;
    }
}
