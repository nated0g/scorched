use bevy::prelude::*;
use bevy::text::{TextSection, TextStyle};

use crate::tank::{Angle, HitPoints, Power, Tank, Turret, TANK_COLOR};

#[derive(Component)]
struct UiText;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, update_ui);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        UiText,
        TextBundle::from_sections([
            TextSection::new(
                "Power: ",
                TextStyle {
                    font_size: 20.0,
                    color: TANK_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                color: TANK_COLOR,
                ..default()
            }),
            TextSection::new(
                " Angle: ",
                TextStyle {
                    font_size: 20.0,
                    color: TANK_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                color: TANK_COLOR,
                ..default()
            }),
            TextSection::new(
                " Health: ",
                TextStyle {
                    font_size: 20.0,
                    color: TANK_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                color: TANK_COLOR,
                ..default()
            }),
        ]),
    ));
}

fn update_ui(
    query: Query<(&Power, &Angle), With<Turret>>,
    tank_query: Query<&HitPoints, With<Tank>>,
    mut ui_query: Query<&mut Text, With<UiText>>,
) {
    let (power, angle) = query.single();
    let hit_points = tank_query.single();
    let mut ui_text = ui_query.single_mut();
    ui_text.sections[1].value = format!("{: >4.0}", power.0);
    ui_text.sections[3].value = format!("{: >5.1}", angle.0);
    ui_text.sections[5].value = format!("{: >4.0}%", hit_points.current * 100 / hit_points.max);
}
