use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::geometry::CollisionEventFlags};
use rand::prelude::*;

use crate::hud::UserCoins;

pub struct CoinPlugin;

#[derive(Debug, Component)]
pub struct Coin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coin)
            .add_systems(Update, handle_collision);
    }
}

fn handle_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    coins: Query<(), With<Coin>>,
    mut coin_info: Query<&mut UserCoins>,
) {
    let mut coin_label = coin_info.single_mut();
    let mut remove_entity = |entity: Entity| {
        commands.entity(entity).despawn_recursive();
        coin_label.0 += 1;
    };

    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, flags) if flags.contains(CollisionEventFlags::SENSOR) => {
                if coins.contains(*entity1) {
                    remove_entity(*entity1);
                } else if coins.contains(*entity2) {
                    remove_entity(*entity2);
                }
            }
            _ => {}
        }
    }
}

fn spawn_coin(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = thread_rng();

    for _ in 0..15 {
        let x = rng.gen_range(-30.0..30.0);
        let z = rng.gen_range(-30.0..30.0);

        commands
            .spawn(PbrBundle {
                material: materials.add(Color::YELLOW),
                mesh: meshes.add(Sphere::default().mesh().ico(5).unwrap()),
                transform: Transform::from_xyz(x, 0.6, z),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(GravityScale(0.0))
            .insert(Collider::ball(0.6))
            .insert(Sensor)
            .insert(Coin)
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}
