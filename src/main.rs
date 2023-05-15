pub mod camera;
pub mod ui;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use camera::CameraPlugin;
use std::f32::consts::FRAC_PI_2;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(UiPlugin)
        .add_plugin(CameraPlugin)
        .add_startup_system(setup_scene)
        .add_system(spin)
        .run();
}

#[derive(Component)]
struct Wall;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane {
    //         size: 50.0,
    //         subdivisions: 0,
    //     })),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(100.)),
    //     ..Default::default()
    // });
    // Cube
    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //         transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //         ..Default::default()
    //     },
    //     PickableBundle::default(),
    //     RaycastPickTarget::default(),
    // ));
    // Spawn cubes
    for x in -2..=2 {
        let z = 0.5 + x as f32 * 0.1;
        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::default())),
                transform: Transform::from_xyz(x as f32 * 200.0, 100.0, z)
                    .with_scale(Vec3::splat(100.)), // splat is same as (100, 100, 100)
                material: materials.add(Color::hsl(0.0, 1.0, z).into()),
                ..Default::default()
            },
            PickableBundle::default(),    // <- Makes the mesh pickable.
            RaycastPickTarget::default(), // <- Needed for the raycast backend.
            OnPointer::<DragStart>::target_remove::<Pickable>(), // Disable picking
            OnPointer::<DragEnd>::target_insert(Pickable), // Re-enable picking
            OnPointer::<Drag>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation += drag.delta.extend(0.0) // Make the square follow the mouse
            }),
            OnPointer::<Drop>::commands_mut(|event, commands| {
                commands.entity(event.dropped).insert(Spin(FRAC_PI_2)); // Spin dropped entity
                commands.entity(event.target).insert(Spin(-FRAC_PI_2)); // Spin dropped-on entity
            }),
            Wall {},
        ));
    }
    // Light
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..Default::default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..Default::default()
    // });
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
