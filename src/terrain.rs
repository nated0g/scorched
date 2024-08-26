use bevy::{
    app::{App, Startup},
    asset::Assets,
    math::Vec3,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    sprite::{MaterialMesh2dBundle, SpriteBundle},
    window::Window,
};
use noise::{NoiseFn, Perlin, Seedable};
use rand::RngCore;
use simple_moving_average::{SumTreeSMA, SMA};

use crate::{explosions::Explosion, projectiles::Projectile};

#[derive(Component)]
pub struct Terrain;

#[derive(Component)]
pub struct TerrainTexture;

const BROWN_COLOR: [u8; 4] = [120, 60, 15, 255]; // Brown color for the terrain
const GREEN_COLOR: [u8; 4] = [34, 139, 34, 255]; // Green color for the top of the terrain
const EXPLOSION_COLOR: Color = Color::srgb(1.0, 0.5, 0.0); // Orange color for the explosion
const GREEN_HEIGHT: u32 = 25; // Height of the green part of the terrain
const EXPLOSION_DURATION: f32 = 0.2; // Duration of the explosion effect in seconds

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_terrain)
            .add_systems(FixedUpdate, update_terrain);
    }
}

fn setup_terrain(
    mut commands: Commands,
    window_query: Query<&Window>,
    mut images: ResMut<Assets<Image>>,
) {
    let window = window_query.single();
    // Create the terrain texture
    let texture_size = Extent3d {
        width: window.width() as u32,
        height: window.height() as u32,
        depth_or_array_layers: 1,
    };
    let mut texture = Image::new_fill(
        texture_size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );

    // Generate the initial terrain using Perlin noise
    let mut rng = rand::thread_rng();
    let perlin = Perlin::default().set_seed(rng.next_u32());
    let mut ma = SumTreeSMA::<_, f32, 200>::new();

    for x in 0..texture_size.width {
        let y_raw = perlin.get([x as f64 * 0.005, x as f64 * 0.001]) as f32 * 800.;
        ma.add_sample(y_raw.abs());
        let y = ma.get_average();

        for y_cur in 0..(y as u32) {
            let flipped_y = texture_size.height - 1 - y_cur;
            let index = (flipped_y * texture_size.width + x) as usize * 4;
            if y_cur >= (y as u32).saturating_sub(GREEN_HEIGHT) {
                texture.data[index..index + 4].copy_from_slice(&GREEN_COLOR);
            } else {
                texture.data[index..index + 4].copy_from_slice(&BROWN_COLOR);
            }
        }
    }

    let texture_handle = images.add(texture);

    // Spawn the terrain entity with the texture
    commands.spawn((
        SpriteBundle {
            texture: texture_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Terrain,
        TerrainTexture,
    ));
}

fn update_terrain(
    mut commands: Commands,
    query: Query<(Entity, &mut Transform, &mut Projectile)>,
    mut terrain_query: Query<&Handle<Image>, With<TerrainTexture>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, transform, projectile) in query.iter() {
        // Check for collision with terrain and update the texture
        for texture_handle in terrain_query.iter_mut() {
            if let Some(texture) = images.get_mut(texture_handle) {
                let x = (transform.translation.x + texture.size().x as f32 / 2.0) as u32;
                let y = (transform.translation.y + texture.size().y as f32 / 2.0) as u32;

                if x < texture.size().x && y < texture.size().y {
                    let flipped_y = texture.size().y - 1 - y; // Flip the y-coordinate
                    let index = (flipped_y * texture.size().x + x) as usize * 4;

                    // Check if the projectile hits the terrain
                    if texture.data[index + 3] != 0 {
                        // Create an explosion effect
                        create_explosion(texture, x, flipped_y, projectile.blast_radius);

                        // Spawn the explosion effect
                        commands.spawn((
                            MaterialMesh2dBundle {
                                mesh: meshes
                                    .add(Mesh::from(Circle {
                                        radius: projectile.blast_radius,
                                    }))
                                    .into(),
                                material: materials.add(ColorMaterial::from(EXPLOSION_COLOR)),
                                transform: Transform::from_translation(transform.translation),
                                ..Default::default()
                            },
                            Explosion {
                                timer: Timer::from_seconds(EXPLOSION_DURATION, TimerMode::Once),
                                blast_radius: projectile.blast_radius,
                                scale: 1.0,
                                opacity: 1.0,
                            },
                        ));

                        // Remove the projectile
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}

fn create_explosion(texture: &mut Image, x: u32, y: u32, radius: f32) {
    let width = texture.size().x;
    let height = texture.size().y;
    let radius_squared = radius * radius;

    for dx in -(radius as i32)..=(radius as i32) {
        for dy in -(radius as i32)..=(radius as i32) {
            let distance_squared = (dx * dx + dy * dy) as f32;
            if distance_squared <= radius_squared {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let index = (ny as u32 * width + nx as u32) as usize * 4;
                    texture.data[index] = 0; // R
                    texture.data[index + 1] = 0; // G
                    texture.data[index + 2] = 0; // B
                    texture.data[index + 3] = 0; // A
                }
            }
        }
    }
}

/// Get the terrain height at the given x-coordinate
pub fn get_terrain_height(
    images: &ResMut<Assets<Image>>,
    texture_handle: &Handle<Image>,
    x: f32,
    window_height: f32,
) -> f32 {
    if let Some(texture) = images.get(texture_handle) {
        let width = texture.size().x as f32;
        let height = texture.size().y as f32;
        let x = (x + width / 2.0).clamp(0.0, width - 1.0) as u32;

        for y in 0..height as u32 {
            let index = (y * texture.size().x + x) as usize * 4;
            if texture.data[index + 3] != 0 {
                let flipped_y = height as u32 - 1 - y; // Flip the y-coordinate
                return flipped_y as f32 - height / 2.0;
            }
        }
    }
    -window_height / 2.0
}
