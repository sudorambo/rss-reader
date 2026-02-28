//! Article detail view: title, date, source, formatted body, enclosures with Open/Download (FR-003, FR-009).

use crate::{download_enclosure, format_article, open_enclosure, SubscriptionList};
use eframe::egui;

/// Draw article detail for `selected_item_id`; show "Not found" if item missing (FR-009).
/// Body is scrollable; each enclosure has Open and Download buttons (FR-003).
pub fn show(
    ui: &mut egui::Ui,
    store: &SubscriptionList,
    selected_item_id: Option<&str>,
    selected_feed: Option<&str>,
) {
    let Some(id) = selected_item_id else {
        ui.label("Select an article.");
        return;
    };

    let Some(item) = store.get_item(id, selected_feed) else {
        ui.colored_label(egui::Color32::RED, "Not found.");
        return;
    };

    // Reserve full panel height so the scroll area viewport fills the space and scrolls when content is long.
    let available_height = ui.available_height();
    ui.set_min_height(available_height);
    egui::ScrollArea::vertical()
        .auto_shrink([false, false]) // don't shrink; keep viewport full height so scrollbar appears
        .max_height(available_height)
        .show(ui, |ui| {
        let date_str = item
            .published
            .as_ref()
            .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "?".to_string());

        ui.heading(&item.title);
        ui.label(format!("Date:   {}", date_str));
        ui.label(format!("Source: {}", item.feed_url));
        if let Some(ref link) = item.link {
            if !link.is_empty() {
                ui.hyperlink_to("Link", link);
            }
        }
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        let body = format_article(item.content.as_deref(), 80);
        ui.label(egui::RichText::new(body).monospace());

        if !item.enclosures.is_empty() {
            ui.add_space(8.0);
            ui.separator();
            ui.label("Media:");
            for (idx, enc) in item.enclosures.iter().enumerate() {
                ui.horizontal(|ui| {
                    let mime = enc.media_type.as_deref().unwrap_or("?");
                    ui.label(format!("[{}] {} ({})", idx, enc.url, mime));
                    if ui.button("Open").clicked() {
                        let _ = open_enclosure(enc);
                    }
                    if ui.button("Download").clicked() {
                        if let Ok(path) = download_enclosure(enc, None) {
                            ui.label(format!("Saved to {}", path.display()));
                        }
                    }
                });
            }
        }
    });
}
