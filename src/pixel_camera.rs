use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::texture::{ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::render::view::RenderLayers;
use bevy::window::WindowResized;

pub struct Plugin(pub &'static u32, pub &'static u32);

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        let res_width = self.0;
        let res_height = self.1;
        app.add_systems(
            Startup,
            |commands: Commands, images: ResMut<Assets<Image>>| {
                setup_camera(commands, images, res_width, res_height)
            },
        )
        .add_systems(
            Update,
            |mut resize_events: EventReader<WindowResized>,
             mut projections: Query<&mut OrthographicProjection, With<OuterCamera>>| {
                for event in resize_events.read() {
                    let h_scale = event.width / *res_width as f32;
                    let v_scale = event.height / *res_height as f32;
                    let mut projection = projections.single_mut();
                    projection.scale = 1. / h_scale.min(v_scale).round();
                }
            },
        );
    }
}

/// Default render layers for pixel-perfect rendering.
/// You can skip adding this component, as this is the default.
const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

/// Render layers for high-resolution rendering.
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.
#[derive(Component)]
struct Canvas;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
struct InGameCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
struct OuterCamera;

fn setup_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    width: &u32,
    height: &u32,
) {
    let canvas_size = Extent3d {
        width: *width,
        height: *height,
        ..default()
    };

    // this Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
            mag_filter: ImageFilterMode::Nearest,
            // min_filter: todo!(),
            // mipmap_filter: todo!(),
            lod_min_clamp: 1.,
            lod_max_clamp: 10.,
            ..Default::default()
        }),
        ..Default::default()
    };

    // fill image.data with zeroes
    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    // this camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
    ));

    // spawn the canvas
    commands.spawn((
        SpriteBundle {
            texture: image_handle,
            ..default()
        },
        Canvas,
        HIGH_RES_LAYERS,
    ));

    // the "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    commands.spawn((Camera2dBundle::default(), OuterCamera, HIGH_RES_LAYERS));
}
