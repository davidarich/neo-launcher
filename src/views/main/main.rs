use chrono::{Local, Timelike};
use eframe::egui;
use log::warn;

use crate::{
    NeoLauncher, config,
    features::clock::{get_clock_color, next_phase_start},
};

// Main Menu View
pub fn render_main_menu(launcher: &mut NeoLauncher, ui: &mut egui::Ui) {
    ui.add_space(8.0);
    // hamburger menu
    ui.menu_button(egui::RichText::new("Menu"), |ui| {
        if ui.button("Reload Config").clicked() {
            match config::load() {
                Ok(cfg) => {
                    launcher.config = cfg;
                }
                Err(e) => {
                    warn!("failed to load config: {}", e);
                    launcher.config = config::AppConfig::default();
                    launcher.ui_state.nav_config_error(e);
                }
            }
        }
        // TODO: open settings window when the settings UI is implemented
        if ui.button("Open Settings").clicked() {
            // no-op for now
        }
    });

    ui.add_space(12.0);
    ui.separator();

    // clock
    // format ref - https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    let local_time = Local::now();
    let time = local_time.format("%r").to_string();
    let clock_color = match get_clock_color(local_time.hour(), &launcher.config.clock) {
        Ok(c) => c,
        Err(e) => {
            warn!("invalid clock color: {}", e);
            launcher.ui_state.nav_config_error(e);
            egui::Color32::WHITE
        }
    };
    ui.vertical_centered(|ui| {
        // day, month, date, year
        ui.label(
            egui::RichText::new(local_time.format("%A %B %d %Y").to_string())
                .heading()
                .size(14.0),
        );
        // time
        ui.label(
            egui::RichText::new(time)
                .heading()
                .size(36.0)
                .color(clock_color),
        );
        // countdown to the start of the next clock phase
        if let Some((hours, minutes, phase_name)) =
            next_phase_start(&local_time, &launcher.config.clock)
        {
            ui.label(
                egui::RichText::new(format!("{} in {}h {:02}m", phase_name, hours, minutes))
                    .size(12.0),
            );
        }
    });
    ui.separator();
    ui.add_space(12.0);

    let mut workspace_count = 0;
    for workspace in &launcher.config.workspaces {
        if !workspace.is_hidden {
            let workspace_icon = egui::Image::new(workspace.icon_path.to_string())
                .fit_to_exact_size(egui::Vec2 { x: 64.0, y: 64.0 });
            let workspace_button = egui::Button::new(workspace_icon).corner_radius(4);
            if ui.add(workspace_button).clicked() {
                launcher.ui_state.nav_workspace_menu(workspace_count);
            }
            ui.add_space(8.0);
        }
        workspace_count += 1;
    }
}
