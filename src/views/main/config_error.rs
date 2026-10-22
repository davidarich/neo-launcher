use eframe::egui;
use log::warn;

use crate::{NeoLauncher, config};

// Config Error View
pub fn render_config_error(launcher: &mut NeoLauncher, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new("An error has occurred")
                .heading()
                .size(20.0),
        );
        ui.add_space(4.0);
        if let Some(err) = &launcher.ui_state.config_error {
            ui.label(egui::RichText::new(err.clone()).size(14.0));
            ui.add_space(4.0);
        }
        if ui.button("Reload Config").clicked() {
            match config::load() {
                Ok(cfg) => {
                    launcher.config = cfg;
                    launcher.ui_state.nav_main_menu();
                }
                Err(e) => {
                    warn!("failed to load config: {}", e);
                    launcher.config = config::AppConfig::default();
                    launcher.ui_state.config_error = Some(e);
                }
            }
        }
    });
}
