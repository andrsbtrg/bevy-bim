use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub const SIDE_PANEL_DEFAULT_WIDTH: f32 = 200.;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EguiPlugin)
            .init_resource::<OccupiedScreenSpace>()
            .add_system(ui_system);
    }
}

#[allow(dead_code)]
#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

fn ui_system(mut contexts: EguiContexts, mut occupied_screen_space: ResMut<OccupiedScreenSpace>) {
    let ctx = contexts.ctx_mut();

    // Top panel
    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
    egui::TopBottomPanel::top("top_panel")
        .default_height(20.)
        .show(ctx, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    // quit
                    // ToDo! find a better way to exit
                    std::process::exit(0);
                }
            })
        });

    // Side panel
    occupied_screen_space.right = egui::SidePanel::left("left_panel")
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
