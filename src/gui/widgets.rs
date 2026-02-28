//! Shared widgets: loading indicator, error banner.

use eframe::egui;

/// Show a dismissible error banner if `message` is `Some`.
pub fn show_error_banner(ui: &mut egui::Ui, message: &mut Option<String>) {
    if let Some(ref msg) = *message {
        let mut dismiss = false;
        ui.horizontal(|ui| {
            ui.colored_label(egui::Color32::RED, "Error:");
            ui.label(msg.as_str());
            dismiss = ui.button("Dismiss").clicked();
        });
        if dismiss {
            *message = None;
        }
    }
}

/// Show a loading indicator (spinner or text). Used during add-feed and refresh (T017, T020).
pub fn show_loading(ui: &mut egui::Ui) {
    ui.spinner();
    ui.label("Loading…");
}
