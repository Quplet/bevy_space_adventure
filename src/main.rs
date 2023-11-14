use bevy::prelude::*;

use crate::player::*;
use crate::star::*;

mod player;
mod star;
//struct Asteroid;

#[derive(Resource)]
pub struct GameStats {
    pub speed: f32,
    acceleration: f32
}

impl Default for GameStats {
    fn default() -> Self {
        GameStats { speed: 1., acceleration: 0.01 }
    }
}

fn setup(mut commands: Commands) {
    
    //let asteroid_handle = asset_server.load("res/textures/asteroid.png");
    commands.spawn(Camera2dBundle::default());
}

fn update_game_stats(time: Res<Time>, mut game_stats: ResMut<GameStats>) {
    game_stats.speed += game_stats.acceleration * time.delta_seconds();
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Space Adventure".into(),
            resolution: (1280., 720.).into(),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::rgb(0.0001, 0., 0.001)))
    .insert_resource(GameStats::default())
    .add_systems(Startup, (setup, player_setup, stars_setup))
    .add_systems(Update, (player_movement, star_movement, update_game_stats))
    .run();
}
