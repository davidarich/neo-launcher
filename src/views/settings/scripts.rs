use eframe::egui;

use crate::config;

/// UI state for the Scripts settings view.
///
/// egui is immediate-mode, so anything that needs to persist across frames
/// (the selected script, the name being entered, whether the "add" row is
/// visible) lives here rather than as a local variable in the render function.
#[derive(Clone, Debug)]
pub struct ScriptsUIState {
    /// Index of the currently selected script in `config::AppConfig::scripts`,
    /// or `None` when no script is selected.
    pub selected_script: Option<usize>,
    /// Name entered in the "add script" input field.
    pub new_script_name: String,
    /// Whether the "new script" popup is open.
    pub show_add_script: bool,
    /// Whether the "delete script" confirmation popup is open.
    pub show_delete_script: bool,
}

impl Default for ScriptsUIState {
    fn default() -> Self {
        Self {
            selected_script: None,
            new_script_name: String::new(),
            show_add_script: false,
            show_delete_script: false,
        }
    }
}

pub fn render_scripts_menu(
    ui: &mut egui::Ui,
    config: &mut config::AppConfig,
    state: &mut ScriptsUIState,
) {
    ui.label(egui::RichText::new("Scripts").size(18.0));
    ui.add_space(2.0);

    // Script selector combo box; disabled when there are no scripts.
    // `ComboBox` does not implement `Widget`, so the disabled case shows a
    // plain disabled button instead.
    if config.scripts.is_empty() {
        ui.add_enabled(false, egui::Button::new("No scripts"));
    } else {
        let mut selected = state.selected_script.unwrap_or(0);
        if selected >= config.scripts.len() {
            selected = 0;
        }
        egui::ComboBox::new("script_selector", "Script:").show_index(
            ui,
            &mut selected,
            config.scripts.len(),
            |i| config.scripts[i].name.clone(),
        );
        state.selected_script = Some(selected);
    }

    ui.add_space(4.0);

    // "Delete Script" button toggles the confirmation popup window.
    if ui.button("Delete Script").clicked() {
        if state.selected_script.is_some() {
            state.show_delete_script = !state.show_delete_script;
        }
    }

    // "New Script" button toggles the popup window for entering a script name.
    if ui.button("New Script").clicked() {
        state.show_add_script = !state.show_add_script;
    }

    if state.show_add_script {
        // `Window::open` borrows the flag for the whole `show` call, so a
        // local mirror is used and synced back below.
        let mut show = true;
        let mut done = false;
        egui::Window::new("New Script")
            .open(&mut show)
            .resizable(false)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.label("Name:");
                ui.add(egui::TextEdit::singleline(&mut state.new_script_name));
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    if ui.button("Add").clicked() {
                        let name = state.new_script_name.trim().to_string();
                        if !name.is_empty() {
                            config.scripts.push(config::Script {
                                name,
                                value: String::new(),
                            });
                            // Select the newly added script.
                            state.selected_script = Some(config.scripts.len() - 1);
                            state.new_script_name = String::new();
                            done = true;
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        state.new_script_name = String::new();
                        done = true;
                    }
                });
            });
        state.show_add_script = show && !done;
    }

    if state.show_delete_script {
        let mut show = true;
        let mut done = false;
        let script_name = state
            .selected_script
            .filter(|i| *i < config.scripts.len())
            .map(|i| config.scripts[i].name.clone())
            .unwrap_or_default();
        egui::Window::new("Delete Script")
            .open(&mut show)
            .resizable(false)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.label(format!(
                    "Are you sure you want to delete the script named {}",
                    script_name
                ));
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    if ui.button("Yes").clicked() {
                        if let Some(idx) = state.selected_script {
                            if idx < config.scripts.len() {
                                config.scripts.remove(idx);
                                state.selected_script = None;
                            }
                        }
                        done = true;
                    }
                    if ui.button("No").clicked() {
                        done = true;
                    }
                });
            });
        state.show_delete_script = show && !done;
    }

    // Code editor for the selected script; disabled when none is selected.
    ui.add_space(8.0);
    ui.label(egui::RichText::new("Script Code").size(14.0));
    ui.add_space(2.0);

    match state.selected_script {
        Some(idx) if idx < config.scripts.len() => {
            ui.add(
                egui::TextEdit::multiline(&mut config.scripts[idx].value)
                    .code_editor()
                    .desired_width(f32::INFINITY),
            );
        }
        _ => {
            let mut placeholder = String::new();
            ui.add_enabled(
                false,
                egui::TextEdit::multiline(&mut placeholder)
                    .code_editor()
                    .desired_width(f32::INFINITY),
            );
        }
    }
}
