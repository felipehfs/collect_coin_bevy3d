use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::geometry::CollisionEventFlags};
use rand::prelude::*;

use crate::hud::UserCoins;

const MAX_COIN: u32 = 5;

pub struct CoinPlugin;

#[derive(Debug, Component)]
pub struct Coin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coin)
            .add_systems(Update, handle_collision)
            .add_systems(Update, handle_game_over);
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

fn handle_game_over(mut commands: Commands, total_coins_query: Query<(), With<Coin>>) {
    let total = total_coins_query.iter().len();
    if total == 0 {
        commands.spawn(TextBundle::from_section("You Win!", TextStyle { font_size: 48.0, color: Color::YELLOW, ..default() }).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(125.0),
            left: Val::Px(135.0),
            ..default()
        }));
    }   
}

fn spawn_coin(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = thread_rng();

    for _ in 0..MAX_COIN {
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
