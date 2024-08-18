use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use noise::{NoiseFn, Perlin, PerlinSurflet, Seedable};
use rand::RngCore;

/*
                ⠀⠀⠀⠀⠀⢀⣠⣤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⠀⠀⢀⣴⠿⢫⣯⣍⣉⠙⢷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⠀⣰⠟⣱⡾⠛⢿⣿⣿⣷⠀⣿⡻⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⣸⡏⣼⣿⣿⣷⣤⡉⠻⡿⠀⡟⠿⡎⠻⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⣿⠀⡟⠳⣽⡻⣿⣿⡾⠃⣼⠃⠀⠉⠲⣌⢻⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⢿⡄⠻⣦⣬⣽⡾⠋⣠⡞⠙⢢⠀⠑⢦⡀⣸⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⠈⠻⣶⣤⣤⣤⣶⢿⡉⠀⠀⠀⡀⠈⠙⣿⣿⢱⠈⠻⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⠀⠀⠀⠉⠛⠷⣌⡓⠯⡳⢄⡀⠈⢳⣴⡿⢻⣿⠄⠀⠈⠻⢦⡄⠀⠀⠀⢀⣀⣀⣀⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⠀⠀⠀⠀⠀⠀⠈⠙⠳⢦⠤⠵⢾⣿⣯⣿⣷⡃⠀⠀⠀⠀⠀⠙⢿⣟⠛⣋⣉⣉⣉⣉⣉⡉⠉⠉⢙⡟⠳⢦⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⢯⣕⢯⠲⣄⠀⠀⠀⠀⠀⠙⢿⣿⠟⠿⠻⠿⠿⢭⡇⠀⢸⠃⣰⣤⣌⡙⠳⢦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⢶⣄⠙⠶⣔⠦⣄⠀⠓⣾⣷⠀⠀⠀⠈⢸⡇⠀⣾⠀⡟⠒⠈⠉⠓⠶⣬⣹⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣿⣷⣦⣈⡓⢈⣓⣶⣿⠋⡄⠀⠀⠀⣿⠀⢀⡇⣠⣇⣀⣀⣀⣀⡐⢺⡇⢿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⠛⣿⣿⣿⡟⠛⠿⠛⠋⢁⣴⣁⣤⠤⠶⠿⠶⠾⠿⠿⢿⣿⣿⣿⣿⣿⣿⣿⢾⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡴⠶⠶⠶⠛⠛⢻⡿⣿⡿⠷⠿⠛⠛⠛⠛⠛⠛⢩⡟⡁⢠⠖⠀⠀⡀⣠⡶⠻⣷⣄⣈⠉⠛⠻⠿⣼⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀
                ⠀⠀⠀⠀⠀⠀⠀⢀⡼⠋⣠⢢⡀⢠⠆⣴⣿⡾⢋⡄⠀⠀⠀⡴⠀⠀⠀⣴⣋⣼⣵⣃⣀⣤⣾⡿⠋⣠⡾⢻⣯⡙⠓⢶⣦⣤⣈⠙⠛⢦⣤⡀⠀⠀⠀⠀⠀
                ⠀⠀⠀⠀⠀⠀⢠⣾⠥⢾⠷⢿⣾⡿⣿⣿⡟⢁⡞⠀⡤⢀⣞⡇⠀⣠⠚⢻⣤⣮⣤⣤⣽⣽⣽⣷⡾⣿⣶⣿⠿⠿⠿⣿⡷⣤⣍⣻⠶⣦⣈⠙⢷⣤⡀⠀⠀
                ⠀⠀⠀⠀⠀⠀⠀⢹⡷⡶⢺⣿⡛⡿⣻⢿⡶⠿⠺⡟⠛⠋⠉⠉⢻⡉⢹⣿⣦⣴⣤⣿⣦⣾⣾⣫⡴⣿⠟⣴⢾⣿⣶⣌⢻⣦⣍⡙⠳⣦⣍⡿⢦⡈⠻⣦⡄
                ⠀⠀⠀⠀⠀⠀⠀⢿⡗⠻⣿⣻⠻⡛⢻⡌⢻⡄⠀⠓⠀⠘⣦⡄⠀⠓⠈⢿⡘⠈⢯⠻⡼⠈⢿⡈⢷⣿⠀⣿⣿⣉⣿⣿⢈⡿⢿⡿⣿⢿⣿⣿⣦⣙⣷⣼⠇
                ⠀⠀⠀⠀⠀⠀⠀⠈⢿⣄⣹⣥⣷⣧⣄⣿⣾⢿⡄⢷⡀⠀⠸⡾⡄⢠⡸⣌⣿⣤⣬⢦⡿⢤⡼⣿⠟⣿⣆⠹⢿⣿⡿⢋⣼⣇⣸⣿⣬⣻⣿⣿⣿⣿⣧⠛⣦
                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⡄⣤⡽⡆⡄⢹⣄⢻⣟⣿⣿⣿⣿⢛⣿⣿⣿⣿⣷⡹⢬⡃⢀⢣⠘⣇⠘⣟⢳⣶⣦⠶⣿⣿⣾⣿⣿⣿⣿⣏⣿⣾⣿⣿⠿⣿
                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡶⠾⠷⢷⠾⠶⡿⣟⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡶⠷⣾⠷⣶⡾⣟⠹⣿⣶⣿⠿⢿⣿⣿⣿⠿⣷⣿⡿⣿⣿⠿⣧⡾⠃
                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣆⢲⡈⢳⡄⠁⠹⣆⡽⠿⠿⢿⡿⢿⠿⢿⠿⢿⣿⣷⣄⣘⠲⡜⠁⠻⣤⡿⠙⠋⢻⡏⠉⠙⣿⠉⠙⣟⠉⢹⣧⣴⠏⠀⠀
                    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠁⠀⠀⠀

 ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄         ▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄
▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░▌       ▐░▌▐░░░░░░░░░░░▌▐░░░░░░░░░░▌
▐░█▀▀▀▀▀▀▀▀▀ ▐░█▀▀▀▀▀▀▀▀▀ ▐░█▀▀▀▀▀▀▀█░▌▐░█▀▀▀▀▀▀▀█░▌▐░█▀▀▀▀▀▀▀▀▀ ▐░▌       ▐░▌▐░█▀▀▀▀▀▀▀▀▀ ▐░█▀▀▀▀▀▀▀█░▌
▐░▌          ▐░▌          ▐░▌       ▐░▌▐░▌       ▐░▌▐░▌          ▐░▌       ▐░▌▐░▌          ▐░▌       ▐░▌
▐░█▄▄▄▄▄▄▄▄▄ ▐░▌          ▐░▌       ▐░▌▐░█▄▄▄▄▄▄▄█░▌▐░▌          ▐░█▄▄▄▄▄▄▄█░▌▐░█▄▄▄▄▄▄▄▄▄ ▐░▌       ▐░▌
▐░░░░░░░░░░░▌▐░▌          ▐░▌       ▐░▌▐░░░░░░░░░░░▌▐░▌          ▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░▌       ▐░▌
 ▀▀▀▀▀▀▀▀▀█░▌▐░▌          ▐░▌       ▐░▌▐░█▀▀▀▀█░█▀▀ ▐░▌          ▐░█▀▀▀▀▀▀▀█░▌▐░█▀▀▀▀▀▀▀▀▀ ▐░▌       ▐░▌
          ▐░▌▐░▌          ▐░▌       ▐░▌▐░▌     ▐░▌  ▐░▌          ▐░▌       ▐░▌▐░▌          ▐░▌       ▐░▌
 ▄▄▄▄▄▄▄▄▄█░▌▐░█▄▄▄▄▄▄▄▄▄ ▐░█▄▄▄▄▄▄▄█░▌▐░▌      ▐░▌ ▐░█▄▄▄▄▄▄▄▄▄ ▐░▌       ▐░▌▐░█▄▄▄▄▄▄▄▄▄ ▐░█▄▄▄▄▄▄▄█░▌
▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░▌       ▐░▌▐░░░░░░░░░░░▌▐░▌       ▐░▌▐░░░░░░░░░░░▌▐░░░░░░░░░░▌
 ▀▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀▀  ▀         ▀  ▀▀▀▀▀▀▀▀▀▀▀  ▀         ▀  ▀▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀


This is a rough clone of my favourite old DOS game, Scorched Earth. It's a work in progress, not playable yet.
*/

use simple_moving_average::{SumTreeSMA, SMA};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<FireProjectile>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                rotate_turret,
                fire_projectile,
                move_projectile,
                adjust_power,
                update_ui,
            ),
        )
        .run();
}

#[derive(Component)]
struct UiText;

#[derive(Component)]
struct Tank;

#[derive(Component)]
struct Angle(f32);

#[derive(Component)]
struct Power(f32);

// #[derive(Component)]
// struct Gravity(f32);

// #[derive(Component)]
// struct Wind(f32);

#[derive(Component)]
struct Projectile {
    velocity: Vec2,
}

const MINIMUM_POWER: f32 = 250.;

#[derive(Component)]
struct Terrain;

impl Projectile {
    fn new((angle, power): (&Angle, &Power)) -> Self {
        let angle = angle.0.to_radians();
        let velocity = Vec2::new(angle.cos(), angle.sin()) * (power.0 + MINIMUM_POWER);
        Projectile { velocity }
    }
}

#[derive(Event, Default)]
struct FireProjectile;

#[derive(Component)]
struct Turret;

const TANK_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
const TURRET_COLOR: Color = Color::srgb(0.1, 0.7, 0.1);
const TANK_RADIUS: f32 = 8.;
const TURRET_LENGTH: f32 = TANK_RADIUS * 2.;
const TURRET_WIDTH: f32 = TANK_RADIUS / 2.;

const GRAVITY: f32 = -980.;

fn setup(
    mut commands: Commands,
    window_query: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.single();
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Tank body
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(CircularSector::from_degrees(TANK_RADIUS, 180.))
                    .into(),
                material: materials.add(TANK_COLOR),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..default()
            },
            Tank,
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
        ]),
    ));
    let window_width = window.width();
    // generate the terrain with perlin noise
    let mut rng = rand::thread_rng();
    let perlin = Perlin::default().set_seed(rng.next_u32());

    let mut ma = SumTreeSMA::<_, f32, 200>::new();

    let x_terrain_granularity = 1.;
    for x in (-(window_width / 2. * x_terrain_granularity) as i32
        ..(window_width / 2. * x_terrain_granularity) as i32)
        .step_by(x_terrain_granularity as usize)
    {
        let y_raw = perlin.get([x as f64 * 0.005, x as f64 * 0.001]) as f32 * 800.;

        ma.add_sample(y_raw.abs());
        let y = ma.get_average();
        dbg!(y, y_raw);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Rectangle::from_size(Vec2::new(x_terrain_granularity, y)))
                    .into(),
                material: materials.add(ColorMaterial::default()),
                transform: Transform::from_translation(Vec3::new(
                    x as f32,
                    -(window.height() / 2.) + (y.abs() / 2.),
                    0.0,
                )),
                ..default()
            },
            Terrain,
        ));
    }
}

const TURRET_ROTATION_SPEED: f32 = 0.5; // rotations per second

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

fn update_ui(
    query: Query<(&Power, &Angle), With<Turret>>,
    mut ui_query: Query<&mut Text, With<UiText>>,
) {
    let (power, angle) = query.single();
    let mut ui_text = ui_query.single_mut();
    ui_text.sections[1].value = format!("{: >4.0}", power.0);
    ui_text.sections[3].value = format!("{:.1}", angle.0);
}
