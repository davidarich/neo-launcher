use eframe::egui;

use crate::{NeoLauncher, SettingsView};

pub fn render_settings_menu(launcher: &mut NeoLauncher, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Settings").heading().size(20.0));
    egui::Panel::left("settings").show(ui, |ui| {
        ui.vertical_centered_justified(|ui| {
            ui.add_space(2.0);
            if ui.button("Main Menu").clicked() {
                launcher.settings_ui_state.nav_main_menu();
            }
            ui.add_space(2.0);
            if ui.button("Workspaces").clicked() {
                launcher.settings_ui_state.nav_workspaces();
            }
            ui.add_space(2.0);
            if ui.button("Scripts").clicked() {
                launcher.settings_ui_state.nav_scripts();
            }
        });
    });

    if matches!(
        launcher.settings_ui_state.selected_view,
        SettingsView::MainMenu
    ) {
        crate::views::settings::main_menu::render_main_menu(ui)
    } else if matches!(
        launcher.settings_ui_state.selected_view,
        SettingsView::Workspaces
    ) {
        crate::views::settings::workspaces::render_workspaces_menu(ui)
    } else if matches!(
        launcher.settings_ui_state.selected_view,
        SettingsView::Scripts
    ) {
        crate::views::settings::scripts::render_scripts_menu(
            ui,
            &mut launcher.settings_ui_state.modified_config,
            &mut launcher.settings_ui_state.scripts_state,
        )
    }

    ui.horizontal(|ui| {
        if ui.button("Close").clicked() {
            launcher.close_settings_window();
        }
        if ui.button("Save").clicked() {
            launcher.save_settings();
        }
        if ui.button("Save & Close").clicked() {
            launcher.save_settings();
            launcher.close_settings_window();
        }
    });

    // TODO: add a horizontal container with right alignment
}
