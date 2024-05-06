pub mod ui;
mod style;

use std::f32::consts::FRAC_PI_2;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use bevy_mod_picking::{events::{Drag, DragEnd, DragStart, Drop, Pointer}, picking_core::Pickable, prelude::On, DefaultPickingPlugins, PickableBundle};
use bevy_panorbit_camera::PanOrbitCameraPlugin;

use ui::UiPlugin;

#[derive(Component)]
struct MyGameCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(UiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, spin)
        .run();
}

// cube
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn cubes
    for x in -2..=2 {
        let z = 0.5 + x as f32 * 0.1;
        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(Cuboid::default())),
                transform: Transform::from_xyz(x as f32 * 200.0, 100.0, z)
                    .with_scale(Vec3::splat(100.)), // splat is same as (100, 100, 100)
                material: materials.add(Color::hsl(0.0, 1.0, z)),
                ..Default::default()
            },
            PickableBundle::default(), // <- Makes the mesh pickable.
            On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
            On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
            On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation.x += drag.delta.x; // Make the square follow the mouse
                transform.translation.y -= drag.delta.y;
            }),
            On::<Pointer<Drop>>::commands_mut(|event, commands| {
                commands.entity(event.dropped).insert(Spin(FRAC_PI_2)); // Spin dropped entity
                commands.entity(event.target).insert(Spin(-FRAC_PI_2)); // Spin dropped-on entity
            }),
        ));
    }
    // ground plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::new([10000.0, 0., 0.].into())),
            material: materials.add(Color::hex("#0074d9").unwrap()),
            ..default()
        },
        PickableBundle::default(),
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        bevy_panorbit_camera::PanOrbitCamera::default(),
    ));
}

pub struct SpinPlugin;

impl Plugin for SpinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spin)
            .add_systems(Update, spin_cleanup);
    }
}

#[derive(Component)]
struct Spin(f32);

fn spin(mut square: Query<(&mut Spin, &mut Transform)>) {
    for (mut spin, mut transform) in square.iter_mut() {
        transform.rotation = Quat::from_rotation_z(spin.0);
        let delta = -spin.0.clamp(-1.0, 1.0) * 0.05;
        spin.0 += delta;
    }
}

fn spin_cleanup(mut square: Query<(Entity, &Spin, &mut Transform)>, mut commands: Commands) {
    for (entity, spin, mut transform) in square.iter_mut() {
        if spin.0.abs().le(&0.001) {
            transform.rotation = Quat::default(); // <- reset the rotation to zero when it's visually neglible
            commands.entity(entity).remove::<Spin>(); // <- remove the component so it's stopped updating
        }
    }
}
/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}
