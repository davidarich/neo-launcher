use eframe::egui;

pub fn render_main_menu(ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Main Menu").size(18.0));
    ui.add_space(2.0);
}
