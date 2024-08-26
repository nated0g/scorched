use bevy::prelude::*;

use crate::tank::{HitPoints, Tank};

// TODO: Make a "new" impl
#[derive(Component)]
pub struct Explosion {
    pub timer: Timer,
    pub blast_radius: f32,
    pub scale: f32,
    pub opacity: f32,
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, explosion);
    }
}

fn explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut Explosion,
            &mut Transform,
            &Handle<ColorMaterial>,
        ),
        Without<Tank>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tank_query: Query<(&mut Transform, &mut HitPoints), With<Tank>>,
) {
    for (entity, mut explosion, mut transform, material_handle) in query.iter_mut() {
        explosion.timer.tick(time.delta());

        // Calculate the progress of the explosion (0.0 to 1.0)
        let progress = explosion.timer.fraction();

        // Update the scale and opacity based on the progress
        explosion.scale = 1.0 + progress * 2.0; // Expand the explosion
        explosion.opacity = 1.0 - progress; // Fade out the explosion

        // Apply the scale and opacity to the transform and material
        transform.scale = Vec3::splat(explosion.scale);
        if let Some(material) = materials.get_mut(material_handle) {
            material.color.set_alpha(explosion.opacity);
        }

        if explosion.timer.finished() {
            // Apply damage to tanks within the blast radius
            for (tank_transform, mut hit_points) in tank_query.iter_mut() {
                let distance = transform.translation.distance(tank_transform.translation);
                if distance < explosion.blast_radius {
                    hit_points.current = hit_points.current.saturating_sub(10); // Decrease hit points by 10
                }
            }

            // Remove the explosion effect
            commands.entity(entity).despawn();
        }
    }
}
