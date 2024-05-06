use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use crate::style;

// use crate::Wall;

pub const SIDE_PANEL_DEFAULT_WIDTH: f32 = 200.;
const SIDE_PANEL_MAX_WIDTH: f32 = 400.;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<OccupiedScreenSpace>()
            .add_plugins(DefaultInspectorConfigPlugin)
            .add_systems(Startup, setup_ui)
            .add_systems(Update, ui_system)
            .add_systems(Update, world_inspector_ui_debug);
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

fn setup_ui(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();
    style::set_style(ctx, style::Theme::dark())
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
                        exit.send(AppExit);
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
