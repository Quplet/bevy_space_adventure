use bevy::prelude::*;

use crate::player::*;
use crate::star::*;
use crate::asteroid::*;

mod player;
mod star;
mod asteroid;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    MainMenu,
    #[default]
    Game,
    Paused
}

#[derive(Resource)]
pub struct GameStats {
    pub speed: f32,
    acceleration: f32
}

impl Default for GameStats {
    fn default() -> Self {
        GameStats { speed: 1., acceleration: 0.02 }
    }
}

fn setup(mut commands: Commands) {
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
    .add_state::<GameState>()
    .add_systems(Startup, (setup, stars_setup))
    .add_systems(OnEnter(GameState::Game), player_setup)
    .add_systems(Update, (star_movement, asteroid_movement).run_if(not(in_state(GameState::Paused)))) 
    .add_systems(Update, (player_movement, update_game_stats, generate_asteroids).run_if(in_state(GameState::Game)))
    .run();
}
