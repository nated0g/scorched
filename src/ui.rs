use bevy::prelude::*;
use bevy::text::{TextSection, TextStyle};

use crate::tank::{Angle, HitPoints, Player, Power, Tank, Turret, TANK_COLOR};

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
                "Player ",
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
                "'s Turn | Power: ",
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
    query: Query<(&Power, &Angle, &Parent), With<Turret>>,
    tank_query: Query<(&HitPoints, &Player), With<Tank>>,
    mut ui_query: Query<&mut Text, With<UiText>>,
) {
    let mut ui_text = ui_query.single_mut();

    // Find the active player and their turret
    for (hit_points, player) in tank_query.iter() {
        if player.is_active {
            // Update player number
            ui_text.sections[1].value = format!("{}", player.id + 1);

            // Find the corresponding turret
            for (power, angle, parent) in query.iter() {
                if let Ok((tank_hit_points, _)) = tank_query.get(parent.get()) {
                    if std::ptr::eq(tank_hit_points, hit_points) {
                        ui_text.sections[3].value = format!("{: >4.0}", power.0);
                        ui_text.sections[5].value = format!("{: >5.1}", angle.0);
                        ui_text.sections[7].value =
                            format!("{: >4.0}%", hit_points.current * 100 / hit_points.max);
                        break;
                    }
                }
            }
            break;
        }
    }
}
