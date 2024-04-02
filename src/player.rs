use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::camera::MainCamera;

const PLAYER_SPEED: f32 = 20.0;

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, follow_player).chain());
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController {
            up: Vec3::Z,
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Player);
}

fn player_movement(
    mut controllers: Query<&mut KinematicCharacterController>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut player = controllers.single_mut();

    let mut movement = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) {
        movement.x += PLAYER_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        movement.x -= PLAYER_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement.z += PLAYER_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement.z -= PLAYER_SPEED * time.delta_seconds();
    }

    player.translation = Some(movement);
}

fn follow_player(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let player_position = player_query.single();
    for mut transform in camera_query.iter_mut() {
        transform.translation = player_position.translation + Vec3::new(0.0, 10.0, -10.0);
        transform.look_at(player_position.translation, Vec3::Y);
    }
}

