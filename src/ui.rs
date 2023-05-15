use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

// use crate::Wall;

pub const SIDE_PANEL_DEFAULT_WIDTH: f32 = 200.;
const SIDE_PANEL_MAX_WIDTH: f32 = 400.;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EguiPlugin)
            .init_resource::<OccupiedScreenSpace>()
            .add_plugin(DefaultInspectorConfigPlugin)
            .add_system(ui_system)
            .add_system(world_inspector_ui_debug); // not working
    }
}

#[allow(dead_code)]
#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

fn ui_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut exit: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();

    // Top panel
    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
    egui::TopBottomPanel::top("top_panel")
        .default_height(20.)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        // quit
                        exit.send(AppExit)
                    }
                })
            })
        });

    // Side panel
    occupied_screen_space.right = egui::SidePanel::left("left_panel")
        .default_width(SIDE_PANEL_DEFAULT_WIDTH)
        .max_width(SIDE_PANEL_MAX_WIDTH)
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Tools");

            if ui.small_button("Cube").clicked() {
                println!("Pressed Cube!");
            }

            if ui.small_button("Sphere").clicked() {
                println!("Pressed Sphere!");
            }

            if ui.small_button("Plane").clicked() {
                println!("Pressed Plane!");
            }

            if ui.small_button("Light").clicked() {
                println!("Pressed Light!");
            }

            // ui.label("Walls");
            // for (wall_i, _) in wall_queries.iter().enumerate() {
            //     ui.selectable_label(false, format!("Wall {wall_i}"));
            // }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
fn world_inspector_ui_debug(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();
    let ctx = egui_context.get_mut();
    // World inspector
    let _right = egui::SidePanel::right("right_panel")
        .exact_width(200.)
        // .default_width(SIDE_PANEL_DEFAULT_WIDTH)
        // .max_width(SIDE_PANEL_MAX_WIDTH)
        .resizable(false)
        .show(ctx, |ui| {
            // materials
            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

            egui::CollapsingHeader::new("Entities").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
            });
        })
        .response
        .rect
        .width();
}
