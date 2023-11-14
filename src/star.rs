use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{thread_rng, Rng};

use crate::GameStats;

#[derive(Component)]
pub struct Star {
    size: f32
}

pub fn stars_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, window: Query<&Window>) {
    let window_res = &window.single().resolution;
    let mut rng = thread_rng();

    for _ in 0..300 {
        let star = Star {
            size: rng.gen_range(0.3..1.0)
        };

        let x = rng.gen_range((-window_res.width()/2.)..(window_res.width()/2.));
        let y = rng.gen_range((-window_res.height()/2.)..(window_res.height()/2.));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(star.size).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(x, y, star.size * 10.)),
                ..default()
            },
            star
        ));
    }
}

pub fn star_movement(time: Res<Time>, game_stats: Res<GameStats>, mut stars: Query<(&Star, &mut Transform)>, window: Query<&Window>) {
    let window_res = &window.single().resolution;
    let mut rng = thread_rng();

    for (star, mut transform) in &mut stars {
        let translation = &mut transform.translation;

        translation.x -= game_stats.speed * 100. * star.size * time.delta_seconds();

        if translation.x + star.size < -window_res.width() / 2. {
            translation.x = window_res.width() / 2. + star.size;
            translation.y = rng.gen_range((-window_res.height()/2.)..(window_res.height()/2.));
        }
    }
}

