mod game;
mod camera;
mod player;
mod platforms;

use bevy::prelude::*;
use game::*;
use camera::*;
use player::*;
use crate::platforms::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(
        WindowPlugin {
            primary_window: Some(Window {
                title: "Doodle Jump game".to_string(),
                resolution: (532., 850.).into(),
                ..default()
            }),
            ..default()
        }
    ))
        .insert_state(GameState::NotStarted)
        .init_resource::<CountStruct>()
        .insert_resource(Paused(false))
        .add_systems(Startup, (background_for_game, camera_setup, setup_count))
        .add_systems(OnEnter(GameState::InProcessGame), (load_platforms, load_player))
        .add_systems(OnEnter(GameState::NotStarted), clean_on_restart)
        .add_systems(OnExit(GameState::NotStarted), clean_start_screen)
        .add_systems(Update, gravity_player
            .run_if(in_state(GameState::InProcessGame))
            .run_if(not_paused))
        .add_systems(Update, player_control_in_x
            .run_if(in_state(GameState::InProcessGame))
            .run_if(not_paused))
        .add_systems(Update, limit_platforms.run_if(in_state(GameState::InProcessGame)))
        .add_systems(Update, (
            collision_player_with_brown_platforms,
            collision_player_with_platforms,
            ))
        .add_systems(Update, borders.run_if(in_state(GameState::InProcessGame)))
        .add_systems(Update, move_camera)
        .add_systems(Update, remove_platforms_below_camera
                                  .run_if(in_state(GameState::InProcessGame)))
        .add_systems(Update, generate_platforms_for_move_camera
            .run_if(in_state(GameState::InProcessGame)))
        .add_systems(Update, (background_follow_camera, distance_between_platforms))
        .add_systems(Update, pause)
        .add_systems(Update, game_over)
        .add_systems(Update, show_game_over)
        .add_systems(Update, show_start_screen)
        .add_systems(Update, restart)
        .add_systems(Update, start_game)
        .add_systems(Update, update_count.run_if(in_state(GameState::InProcessGame)))
        .run();
}
 