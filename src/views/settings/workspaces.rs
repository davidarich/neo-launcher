use eframe::egui;

pub fn render_workspaces_menu(ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Workspaces").size(18.0));
    ui.add_space(2.0);
}
