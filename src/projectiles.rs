use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::Window};

use crate::{
    tank::{Angle, NextTurn, Player, Power, Tank, Turret, TANK_RADIUS},
    GRAVITY,
};

const MINIMUM_POWER: f32 = 200.;

#[derive(Event, Default)]
pub struct FireProjectile;

#[derive(Resource)]
pub struct ProjectileInFlight(pub bool);

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FireProjectile>()
            .insert_resource(ProjectileInFlight(false))
            .add_systems(
                FixedUpdate,
                (fire_projectile, move_projectile, check_projectile_bounds),
            );
    }
}

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2,
    pub blast_radius: f32,
}

impl Projectile {
    fn new((angle, power): (&Angle, &Power)) -> Self {
        let angle = angle.0.to_radians();
        let velocity = Vec2::new(angle.cos(), angle.sin()) * (power.0 + MINIMUM_POWER);
        Projectile {
            velocity,
            blast_radius: 30.,
        }
    }
}

fn fire_projectile(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&Angle, &Power, &GlobalTransform, &Parent), With<Turret>>,
    tank_query: Query<&Player, With<Tank>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventWriter<FireProjectile>,
    mut next_turn_events: EventWriter<NextTurn>,
    mut projectile_in_flight: ResMut<ProjectileInFlight>,
) {
    if projectile_in_flight.0 {
        return;
    }

    for (angle, power, transform, parent) in query.iter() {
        if let Ok(player) = tank_query.get(parent.get()) {
            if !player.is_active {
                continue;
            }

            if keyboard_input.just_pressed(KeyCode::Space) {
                events.send(FireProjectile);
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Rectangle::from_length(TANK_RADIUS / 5.)).into(),
                        material: materials.add(ColorMaterial::default()),
                        visibility: Visibility::Visible,
                        transform: Transform::from_translation(Vec3::new(
                            transform.translation().x,
                            transform.translation().y,
                            -2.,
                        )),
                        ..Default::default()
                    },
                    Projectile::new((angle, power)),
                ));

                projectile_in_flight.0 = true;
                next_turn_events.send(NextTurn);
            }
        }
    }
}

fn move_projectile(mut query: Query<(&mut Transform, &mut Projectile)>, time: Res<Time>) {
    for (mut transform, mut projectile) in query.iter_mut() {
        projectile.velocity.y += GRAVITY * time.delta_seconds();
        transform.translation += projectile.velocity.extend(1.) * time.delta_seconds();
    }
}

fn check_projectile_bounds(
    mut commands: Commands,
    window_query: Query<&Window>,
    query: Query<(Entity, &Transform), With<Projectile>>,
    mut projectile_in_flight: ResMut<ProjectileInFlight>,
) {
    let window = window_query.single();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (entity, transform) in query.iter() {
        let pos = transform.translation;

        // Check if projectile is outside screen bounds with some margin
        if pos.x < -half_width - 50.0
            || pos.x > half_width + 50.0
            || pos.y < -half_height - 50.0
            || pos.y > half_height + 50.0
        {
            commands.entity(entity).despawn();
            projectile_in_flight.0 = false;
        }
    }
}
