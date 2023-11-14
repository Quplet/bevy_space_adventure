use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub number: u8,
    pub movement_speed: f32
}

pub fn player_movement(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut players: Query<(&Player, &mut Transform)>) {
    for (player, mut transform) in &mut players {
        match player.number {
            1=> {
                let mut direction: f32 = 0.0;
                if keyboard_input.pressed(KeyCode::Up) {
                    direction += player.movement_speed;
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    direction -= player.movement_speed;
                }

                transform.translation.y += direction * time.delta_seconds();
            }
            _=> {}
        }
    }
}

pub fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let p1_ship_handle = asset_server.load("textures/ship.png");
    commands.spawn((
        SpriteBundle {
            texture: p1_ship_handle,
            transform: Transform { scale: Vec3 { x: 0.25, y: 0.25, z: 1. }, translation: Vec3 { x: -400., y: 0., z: 11. }, ..default() },
            ..default()
        },
        Player {number: 1, movement_speed: 400.},
    ));
}