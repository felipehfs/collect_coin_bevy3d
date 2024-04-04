use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use coin::CoinPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

mod player;
mod camera;
mod world;
mod coin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Using 3D for practice".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CoinPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin)
        .run();
}
