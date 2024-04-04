use bevy::prelude::*;

pub struct HUDPlugin;

#[derive(Component)]
pub struct UserCoins(pub usize);

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_hud_coins);
    }
}

fn update_hud_coins(mut user_coins: Query<(&mut Text, &UserCoins)>) {
    let (mut text, coin) = user_coins.single_mut();
    text.sections[0].value = format!("Coins: {}", coin.0);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Coins: 0 ",
            TextStyle {
                font_size: 35.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        UserCoins(0),
    ));
}
