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

const TURRET_COLOR: Color = Color::srgb(0.1, 0.7, 0.1);
pub const TANK_RADIUS: f32 = 10.;
const TURRET_LENGTH: f32 = TANK_RADIUS * 2.;
const TURRET_WIDTH: f32 = TANK_RADIUS / 2.;
pub const TURRET_ROTATION_SPEED: f32 = 0.2; // rotations per second
pub const TANK_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FireProjectile>()
            .add_systems(PostStartup, setup) // needs terrain to be loaded
            .add_systems(FixedUpdate, (rotate_turret, adjust_power, apply_gravity));
    }
}

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
    let tank_x = rand::thread_rng().gen_range(-window.width() / 2.0..window.width() / 2.0);
    let tank_y = get_terrain_height(&images, &texture_handle, tank_x, window.height());

    // Tank body
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(CircularSector::from_degrees(TANK_RADIUS, 180.))
                    .into(),
                material: materials.add(TANK_COLOR),
                transform: Transform::from_translation(Vec3::new(tank_x, tank_y, 1.)),
                ..default()
            },
            Tank,
            HitPoints {
                current: 100,
                max: 100,
            },
        ))
        .with_children(|tank| {
            // Turret of the tank
            tank.spawn((
                MaterialMesh2dBundle {
                    // this is the axis of the turret, this simplifies the rotation
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
                // Turret barrel
                turret_axis.spawn((MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Rectangle::from_size(Vec2::new(TURRET_LENGTH, TURRET_WIDTH)))
                        .into(),
                    material: materials.add(TURRET_COLOR),
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
    mut query: Query<(&mut Transform, &mut Angle), With<Turret>>,
    time: Res<Time>,
) {
    for (mut transform, mut angle) in query.iter_mut() {
        let mut rotation_factor = 0.;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            rotation_factor += 1.;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            rotation_factor -= 1.;
        }

        let rotation_speed_degrees = TURRET_ROTATION_SPEED * 360.0; // Convert rotation speed to degrees per second
        let rotation_amount = rotation_factor * rotation_speed_degrees * time.delta_seconds();

        // Update the angle
        angle.0 += rotation_amount;

        if angle.0 > 180. {
            angle.0 = 0.;
        } else if angle.0 < 0. {
            angle.0 = 180.;
        }

        // Apply the rotation in degrees
        transform.rotation = Quat::from_rotation_z(angle.0.to_radians());
    }
}

fn adjust_power(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Power, With<Turret>>,
    time: Res<Time>,
) {
    for mut power in query.iter_mut() {
        let mut power_factor = 0.;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            power_factor += 1.;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            power_factor -= 1.;
        }

        let power_speed = 500.0; // power per second
        let power_amount = power_factor * power_speed * time.delta_seconds();

        // Update the power
        power.0 += power_amount;

        // Clamp to within 0 to 1000
        power.0 = power.0.clamp(0.0, 1000.0);
    }
}

fn apply_gravity(
    mut query: Query<(&mut Transform, &Tank)>,
    terrain_query: Query<&Handle<Image>, With<TerrainTexture>>,
    images: Res<Assets<Image>>,
    time: Res<Time>,
) {
    for (mut transform, _) in query.iter_mut() {
        let mut on_ground = false;

        // Check if the tank is on the ground
        for texture_handle in terrain_query.iter() {
            if let Some(texture) = images.get(texture_handle) {
                let x = (transform.translation.x + texture.size().x as f32 / 2.0) as u32;
                let y = (transform.translation.y + texture.size().y as f32 / 2.0) as u32;

                if x < texture.size().x && y < texture.size().y {
                    let flipped_y = texture.size().y - 1 - y; // Flip the y-coordinate
                    let index = (flipped_y * texture.size().x + x) as usize * 4;

                    // Check if the tank is on the ground
                    if texture.data[index + 3] != 0 {
                        on_ground = true;
                        break;
                    }
                }
            }
        }

        // Apply gravity if the tank is not on the ground
        if !on_ground {
            transform.translation.y += GRAVITY / 2. * time.delta_seconds();
        }
    }
}
