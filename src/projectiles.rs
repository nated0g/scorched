use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    tank::{Angle, Power, Turret, TANK_RADIUS},
    GRAVITY,
};

const MINIMUM_POWER: f32 = 200.;

#[derive(Event, Default)]
pub struct FireProjectile;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FireProjectile>()
            .add_systems(FixedUpdate, (fire_projectile, move_projectile));
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
    query: Query<(&Angle, &Power, &GlobalTransform), With<Turret>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventWriter<FireProjectile>,
) {
    let (angle, power, transform) = query.single();
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
    }
}

fn move_projectile(mut query: Query<(&mut Transform, &mut Projectile)>, time: Res<Time>) {
    for (mut transform, mut projectile) in query.iter_mut() {
        projectile.velocity.y += GRAVITY * time.delta_seconds();
        transform.translation += projectile.velocity.extend(1.) * time.delta_seconds();
    }
}
