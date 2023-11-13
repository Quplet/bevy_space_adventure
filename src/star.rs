use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct Star {
    size: f32
}

pub fn stars_setup(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let star = Star {
            size: rng.gen_range(0.01..1.0)
        };

        let x = rng.gen_range(-200.0..200.0);
        let y = rng.gen_range(-200.0..200.0);

        commands.spawn((MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(star.size).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(x, y, star.size * 10.)),
            ..default()
        },
        star
        ));
    }
}

