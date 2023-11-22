use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::GameStats;

#[derive(Component)]
pub struct Asteroid {
    size: f32,
    rotation_speed: f32
}

pub fn generate_asteroids(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<&Window>) {
    let mut rng = thread_rng();
    let chance: f32 = rng.gen();

    let window_res = &window.single().resolution;

    if chance < 0.0175 {
        let asteroid_handle = asset_server.load("textures/asteroid.png");
        let y_range = (-window_res.height()/2.)..(window_res.height()/2.);

        let asteroid = Asteroid {
            size: rng.gen_range(1.0..2.0),
            rotation_speed: f32::to_radians(rng.gen_range(-180.0..180.0))
        };

        let asteroid_bundle = (
            SpriteBundle {
                texture: asteroid_handle,
                transform: Transform { 
                    translation: Vec3::from((window_res.width()/2. + asteroid.size * 100., rng.gen_range(y_range), 10. + asteroid.size)), 
                    rotation: Quat::from_rotation_z(f32::to_radians(rng.gen_range(0.0..180.0))), 
                    scale: Vec3::from((asteroid.size * 0.1, asteroid.size * 0.1, 1.))
                },
                ..default()
            },
            asteroid
        );

        commands.spawn(asteroid_bundle);
    }
}

pub fn asteroid_movement(mut commands: Commands, time: Res<Time>, game_stats: Res<GameStats>, mut asteroids: Query<(Entity, &Asteroid, &mut Transform)>, window: Query<&Window>) {
    let window_res = &window.single().resolution;

    for (entity, asteroid, mut transform) in &mut asteroids {

        transform.translation.x -= game_stats.speed * 100. * time.delta_seconds();
        transform.rotate_z(asteroid.rotation_speed * time.delta_seconds());

        if transform.translation.x + asteroid.size * 100. < -window_res.width()/2. {
            commands.entity(entity).despawn();
        }
    }
}