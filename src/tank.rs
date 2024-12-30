use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{
    projectiles::FireProjectile,
    terrain::{get_terrain_height, TerrainTexture},
    GRAVITY,
};

#[derive(Component)]
pub struct Tank;

#[derive(Component)]
pub struct Turret;

#[derive(Component)]
pub struct Angle(pub f32);

#[derive(Component)]
pub struct Power(pub f32);

#[derive(Component)]
pub struct HitPoints {
    pub current: u32,
    pub max: u32,
}

#[derive(Component)]
pub struct Player {
    pub id: u32,
    pub is_active: bool,
}

#[derive(Component)]
pub struct FallTracker {
    highest_y: f32,
    is_falling: bool,
}

const TURRET_COLOR: Color = Color::srgb(0.1, 0.7, 0.1);
pub const TANK_RADIUS: f32 = 5.;
const TURRET_LENGTH: f32 = TANK_RADIUS * 2.;
const TURRET_WIDTH: f32 = TANK_RADIUS / 2.2;
pub const TURRET_ROTATION_SPEED: f32 = 0.1; // rotations per second
pub const TANK_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
const PLAYER_COLORS: [Color; 4] = [
    Color::srgb(0.0, 1.0, 0.0), // Green
    Color::srgb(1.0, 0.0, 0.0), // Red
    Color::srgb(0.0, 0.0, 1.0), // Blue
    Color::srgb(1.0, 1.0, 0.0), // Yellow
];

const FALL_DAMAGE_THRESHOLD: f32 = 50.0;
const FALL_DAMAGE_MULTIPLIER: f32 = 0.5;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FireProjectile>()
            .add_event::<NextTurn>()
            .add_systems(PostStartup, setup)
            .add_systems(
                FixedUpdate,
                (
                    rotate_turret,
                    adjust_power,
                    apply_gravity,
                    handle_turns,
                    update_tank_appearance,
                    check_tank_death,
                ),
            );
    }
}

#[derive(Event)]
pub struct NextTurn;

fn setup(
    mut commands: Commands,
    window_query: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    terrain_query: Query<&Handle<Image>, With<TerrainTexture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    images: ResMut<Assets<Image>>,
) {
    let window = window_query.single();
    let texture_handle = terrain_query.single();

    // Spawn two players initially
    for player_id in 0..2 {
        let tank_x = if player_id == 0 {
            // First player starts on the left side
            rand::thread_rng().gen_range(-window.width() / 2.0..(-window.width() / 4.0))
        } else {
            // Second player starts on the right side
            rand::thread_rng().gen_range(window.width() / 4.0..window.width() / 2.0)
        };

        let tank_y = get_terrain_height(&images, texture_handle, tank_x, window.height());
        spawn_tank(
            &mut commands,
            &mut meshes,
            &mut materials,
            tank_x,
            tank_y,
            player_id,
        );
    }
}

fn spawn_tank(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
    player_id: u32,
) {
    let color = PLAYER_COLORS[player_id as usize % PLAYER_COLORS.len()];

    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(CircularSector::from_degrees(TANK_RADIUS, 180.))
                    .into(),
                material: materials.add(color),
                transform: Transform::from_translation(Vec3::new(x, y, 1.)),
                ..default()
            },
            Tank,
            Player {
                id: player_id,
                is_active: player_id == 0, // First player starts
            },
            HitPoints {
                current: 100,
                max: 100,
            },
            FallTracker {
                highest_y: y,
                is_falling: false,
            },
        ))
        .with_children(|tank| {
            // Turret of the tank
            tank.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Rectangle::from_size(Vec2::new(
                            TURRET_LENGTH * 2.,
                            TURRET_WIDTH,
                        )))
                        .into(),
                    material: materials.add(ColorMaterial::default()),
                    visibility: Visibility::Hidden,
                    transform: Transform {
                        translation: Vec3::new(0., TURRET_WIDTH / 2., -1.),
                        ..default()
                    },
                    ..default()
                },
                Turret,
                Angle(0.),
                Power(500.),
            ))
            .with_children(|turret_axis| {
                turret_axis.spawn((MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Rectangle::from_size(Vec2::new(TURRET_LENGTH, TURRET_WIDTH)))
                        .into(),
                    material: materials.add(color),
                    visibility: Visibility::Visible,
                    transform: Transform {
                        translation: Vec3::new(TURRET_LENGTH / 2., 0., 0.),
                        ..default()
                    },
                    ..default()
                },));
            });
        });
}

fn rotate_turret(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Angle, &Parent), With<Turret>>,
    tank_query: Query<&Player, With<Tank>>,
    time: Res<Time>,
) {
    for (mut transform, mut angle, parent) in query.iter_mut() {
        if let Ok(player) = tank_query.get(parent.get()) {
            if !player.is_active {
                continue;
            }

            let mut rotation_factor = 0.;

            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                rotation_factor += 1.;
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                rotation_factor -= 1.;
            }

            let rotation_speed_degrees = TURRET_ROTATION_SPEED * 360.0;
            let rotation_amount = rotation_factor * rotation_speed_degrees * time.delta_seconds();

            angle.0 += rotation_amount;

            if angle.0 > 180. {
                angle.0 = 0.;
            } else if angle.0 < 0. {
                angle.0 = 180.;
            }

            transform.rotation = Quat::from_rotation_z(angle.0.to_radians());
        }
    }
}

fn adjust_power(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Power, &Parent), With<Turret>>,
    tank_query: Query<&Player, With<Tank>>,
    time: Res<Time>,
) {
    for (mut power, parent) in query.iter_mut() {
        if let Ok(player) = tank_query.get(parent.get()) {
            if !player.is_active {
                continue;
            }

            let mut power_factor = 0.;

            if keyboard_input.pressed(KeyCode::ArrowUp) {
                power_factor += 1.;
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                power_factor -= 1.;
            }

            let power_speed = 500.0;
            let power_amount = power_factor * power_speed * time.delta_seconds();

            power.0 += power_amount;
            power.0 = power.0.clamp(0.0, 1000.0);
        }
    }
}

fn handle_turns(
    mut next_turn_events: EventReader<NextTurn>,
    mut tank_query: Query<&mut Player, With<Tank>>,
) {
    for _ in next_turn_events.read() {
        let mut current_player_id = 0;
        let total_players = tank_query.iter().count() as u32;

        // Find the current active player and deactivate them
        for mut player in tank_query.iter_mut() {
            if player.is_active {
                current_player_id = player.id;
                player.is_active = false;
                break;
            }
        }

        // Calculate and activate next player
        let next_player_id = (current_player_id + 1) % total_players;
        for mut player in tank_query.iter_mut() {
            if player.id == next_player_id {
                player.is_active = true;
                break;
            }
        }
    }
}

fn apply_gravity(
    mut query: Query<(&mut Transform, &mut FallTracker, &mut HitPoints), With<Tank>>,
    terrain_query: Query<&Handle<Image>, With<TerrainTexture>>,
    images: Res<Assets<Image>>,
    time: Res<Time>,
) {
    for (mut transform, mut fall_tracker, mut hit_points) in query.iter_mut() {
        let mut on_ground = false;
        let current_y = transform.translation.y;

        if current_y > fall_tracker.highest_y {
            fall_tracker.highest_y = current_y;
            fall_tracker.is_falling = false;
        } else if !fall_tracker.is_falling && current_y < fall_tracker.highest_y {
            fall_tracker.is_falling = true;
        }

        for texture_handle in terrain_query.iter() {
            if let Some(texture) = images.get(texture_handle) {
                let x = (transform.translation.x + texture.size().x as f32 / 2.0) as u32;
                let y = (transform.translation.y + texture.size().y as f32 / 2.0) as u32;

                if x < texture.size().x && y < texture.size().y {
                    let flipped_y = texture.size().y - 1 - y;
                    let index = (flipped_y * texture.size().x + x) as usize * 4;

                    if texture.data[index + 3] != 0 {
                        on_ground = true;

                        if fall_tracker.is_falling {
                            let fall_height = fall_tracker.highest_y - current_y;
                            if fall_height > FALL_DAMAGE_THRESHOLD {
                                let damage = ((fall_height - FALL_DAMAGE_THRESHOLD)
                                    * FALL_DAMAGE_MULTIPLIER)
                                    as u32;
                                hit_points.current = hit_points.current.saturating_sub(damage);
                                info!(
                                    "Fall damage! Height: {}, Damage: {}, Health: {}",
                                    fall_height, damage, hit_points.current
                                );
                            }

                            fall_tracker.is_falling = false;
                            fall_tracker.highest_y = current_y;
                        }
                        break;
                    }
                }
            }
        }

        if !on_ground {
            transform.translation.y += GRAVITY / 2. * time.delta_seconds();
        }
    }
}

fn update_tank_appearance(
    tank_query: Query<(&HitPoints, &Player, Entity), With<Tank>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    children_query: Query<&Children>,
    mut sprite_query: Query<&mut Handle<ColorMaterial>>,
) {
    for (hit_points, player, tank_entity) in tank_query.iter() {
        let health_percentage = hit_points.current as f32 / hit_points.max as f32;
        let base_color = PLAYER_COLORS[player.id as usize % PLAYER_COLORS.len()];

        let damaged_color = Color::srgb(
            base_color.to_srgba().red * health_percentage,
            base_color.to_srgba().green * health_percentage,
            base_color.to_srgba().blue * health_percentage,
        );

        if let Ok(mut material) = sprite_query.get_mut(tank_entity) {
            *material = materials.add(ColorMaterial::from(damaged_color));
        }

        if let Ok(children) = children_query.get(tank_entity) {
            for &child in children.iter() {
                if let Ok(children) = children_query.get(child) {
                    for &turret_part in children.iter() {
                        if let Ok(mut material) = sprite_query.get_mut(turret_part) {
                            *material = materials.add(ColorMaterial::from(damaged_color));
                        }
                    }
                }
            }
        }
    }
}

fn check_tank_death(mut commands: Commands, query: Query<(Entity, &HitPoints), With<Tank>>) {
    for (entity, hit_points) in query.iter() {
        if hit_points.current == 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
