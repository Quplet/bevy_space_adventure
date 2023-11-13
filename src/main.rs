use bevy::prelude::*;

use crate::player::*;
use crate::star::*;

mod player;
mod star;
//struct Asteroid;

#[derive(Resource)]
struct GameStats {
    speed: f32,
    acceleration: f32
}

impl Default for GameStats {
    fn default() -> Self {
        GameStats { speed: 0.5, acceleration: 0.01 }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer> , mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    
    //let asteroid_handle = asset_server.load("res/textures/asteroid.png");
    commands.spawn(Camera2dBundle::default());

    player_setup(&mut commands, &asset_server);
    stars_setup(&mut commands, &mut meshes, &mut materials);
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
    .add_systems(Startup, setup)
    .add_systems(Update, player_movement)
    .add_systems(Update, update_game_stats)
    .run();
}
