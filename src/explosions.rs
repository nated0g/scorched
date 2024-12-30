use bevy::prelude::*;

use crate::{
    projectiles::ProjectileInFlight,
    tank::{HitPoints, NextTurn, Tank},
};

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
    mut explosion_query: Query<
        (Entity, &mut Explosion, &Transform, &Handle<ColorMaterial>),
        Without<Tank>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tank_query: Query<(&GlobalTransform, &mut HitPoints), With<Tank>>,
    mut projectile_in_flight: ResMut<ProjectileInFlight>,
) {
    for (entity, mut explosion, explosion_transform, material_handle) in explosion_query.iter_mut()
    {
        explosion.timer.tick(time.delta());

        let progress = explosion.timer.fraction();
        explosion.scale = 1.0 + progress * 2.0;
        explosion.opacity = 1.0 - progress;

        if let Some(material) = materials.get_mut(material_handle) {
            material.color.set_alpha(explosion.opacity);
        }

        if explosion.timer.finished() {
            info!("Explosion finished, checking for damage");
            for (tank_transform, mut hit_points) in tank_query.iter_mut() {
                let explosion_pos = explosion_transform.translation;
                let tank_pos = tank_transform.translation();
                let distance = explosion_pos.distance(tank_pos);

                info!(
                    "Tank check - Distance: {}, Blast radius: {}",
                    distance, explosion.blast_radius
                );

                if distance <= (explosion.blast_radius * 1.25) {
                    // Calculate damage based on distance from explosion center
                    let damage_multiplier = 1.0 - (distance / (explosion.blast_radius * 2.0));
                    let max_damage = 100;
                    let damage = (max_damage as f32 * damage_multiplier) as u32;

                    let old_health = hit_points.current;
                    hit_points.current = hit_points.current.saturating_sub(damage);

                    info!(
                        "DIRECT HIT! Distance: {}, Damage: {}, Health: {} -> {}",
                        distance, damage, old_health, hit_points.current
                    );
                }
            }

            commands.entity(entity).despawn();
            projectile_in_flight.0 = false;
        }
    }
}
