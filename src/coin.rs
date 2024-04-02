use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::prelude::*;

pub struct CoinPlugin;

#[derive(Debug, Component)]
pub struct Coin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coin)
            .add_systems(FixedUpdate, handle_collision);
    }
}

fn handle_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        info!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.read() {
        info!("Received contact force event: {:?}", contact_force_event);
    }
}

fn spawn_coin(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = thread_rng();

    for _ in 0..5 {
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
            .insert(Sensor::default())
            .insert(Coin)
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}
