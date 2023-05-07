use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<OccupiedScreenSpace>()
        .add_startup_system(spawn_camera)
        .add_system(ui_system)
        .run();
}
pub const SIDE_PANEL_DEFAULT_WIDTH: f32 = 200.;

#[derive(Component)]
pub struct MyCamera;

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    left: f32,
    _top: f32,
    _right: f32,
    _bottom: f32,
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        MyCamera,
    ));
}

fn ui_system(mut contexts: EguiContexts, mut occupied_screen_space: ResMut<OccupiedScreenSpace>) {
    let ctx = contexts.ctx_mut();
    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .default_width(SIDE_PANEL_DEFAULT_WIDTH)
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Left resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
