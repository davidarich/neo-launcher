use eframe::egui;

use crate::{NeoLauncher, features::step};

// Workspace Menu View
pub fn render_workspace_menu(launcher: &mut NeoLauncher, ui: &mut egui::Ui) {
    ui.add_space(8.0);
    if ui.button("< BACK").clicked() {
        launcher.ui_state.nav_main_menu();
    }
    ui.add_space(12.0);
    ui.separator();
    ui.label(
        egui::RichText::new(
            launcher.config.workspaces[launcher.ui_state.workspace_id]
                .name
                .clone(),
        )
        .heading()
        .size(20.0),
    );
    ui.add_space(4.0);
    // actions
    ui.horizontal(|ui| {
        for action in launcher.config.workspaces[launcher.ui_state.workspace_id]
            .actions
            .clone()
        {
            if ui.button(action.name).clicked() {
                let steps = action.steps.clone();
                let cfg = launcher.config.clone();
                launcher.rt.spawn(async move {
                    step::perform_action(steps, cfg);
                });
            }
        }
    });
    ui.add_space(4.0);
    ui.separator();
    ui.add_space(12.0);
    // server clock
    let server_time = chrono::Utc::now().format("%H:%M:%S").to_string();
    ui.label(format!("Server Time: {} ", server_time));
    ui.add_space(8.0);
    // action groups
    for action_group in launcher.config.workspaces[launcher.ui_state.workspace_id]
        .action_groups
        .clone()
    {
        ui.label(
            egui::RichText::new(action_group.name.clone())
                .heading()
                .size(18.0),
        );
        ui.add_space(4.0);
        for group_action in action_group.actions {
            if ui.button(group_action.name).clicked() {
                let steps = group_action.steps.clone();
                let cfg = launcher.config.clone();
                launcher.rt.spawn(async move {
                    step::perform_action(steps, cfg);
                });
            }
            ui.add_space(4.0);
        }
    }
}
