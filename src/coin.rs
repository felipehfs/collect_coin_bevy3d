use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::geometry::CollisionEventFlags};
use rand::prelude::*;

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
    coins: Query<Entity, With<Coin>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, flags)
                if flags.contains(CollisionEventFlags::SENSOR) =>
            {
                info!("A Sensor detected");
                if coins.contains(*entity1) {
                    commands.entity(*entity1).despawn();
                } else if coins.contains(*entity2) {
                    commands.entity(*entity2).despawn()
                }
            }
            _ => {
                info!("Not a sensor detected");
            }
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
            .insert(Collider::ball(0.6))
            .insert(Sensor)
            .insert(Coin)
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}
