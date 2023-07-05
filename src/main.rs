pub mod camera;
pub mod ui;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_mod_picking::{
    prelude::{Click, DefaultPickingPlugins, OnPointer},
    PickableBundle,
};
use camera::CameraPlugin;
use std::f32::consts::PI;

use ui::UiPlugin;
const X_EXTENT: f32 = 14.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(DefaultPickingPlugins)
        .add_plugin(UiPlugin)
        .add_plugin(CameraPlugin)
        .add_startup_system(setup_scene)
        // .add_system(spin)
        .run();
}

#[derive(Component)]
struct Wall;

// cube
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // // Spawn cubes
    // for x in -2..=2 {
    //     let z = 0.5 + x as f32 * 0.1;
    //     commands.spawn((
    //         MaterialMeshBundle {
    //             mesh: meshes.add(Mesh::from(shape::Cube::default())),
    //             transform: Transform::from_xyz(x as f32 * 200.0, 100.0, z)
    //                 .with_scale(Vec3::splat(100.)), // splat is same as (100, 100, 100)
    //             material: materials.add(Color::hsl(0.0, 1.0, z).into()),
    //             ..Default::default()
    //         },
    //         PickableBundle::default(),    // <- Makes the mesh pickable.
    //         RaycastPickTarget::default(), // <- Needed for the raycast backend.
    //         OnPointer::<DragStart>::target_remove::<Pickable>(), // Disable picking
    //         OnPointer::<DragEnd>::target_insert(Pickable), // Re-enable picking
    //         OnPointer::<Drag>::target_component_mut::<Transform>(|drag, transform| {
    //             transform.translation += drag.delta.extend(0.0) // Make the square follow the mouse
    //         }),
    //         OnPointer::<Drop>::commands_mut(|event, commands| {
    //             commands.entity(event.dropped).insert(Spin(FRAC_PI_2)); // Spin dropped entity
    //             commands.entity(event.target).insert(Spin(-FRAC_PI_2)); // Spin dropped-on entity
    //         }),
    //         Wall {},
    //     ));
    // }
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(10000.0).into()),
            material: materials.add(Color::hex("#0074d9").unwrap().into()),
            ..default()
        },
        PickableBundle::default(),
        OnPointer::<Click>::commands_mut(|event, commands| {}),
    ));
    // ground plane
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
    )
}
