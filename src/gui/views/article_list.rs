//! Article list view: items for selected feed (or all), single selection, arrow keys (FR-002).

use crate::SubscriptionList;
use eframe::egui;

/// Draw article list for `selected_feed` (None = all); update `selected_item_id` on click.
/// If no items, show empty state message (FR-008).
/// Set `*focus_tag = Some(article_list_tag)` when user clicks in the list for arrow-key handling.
pub fn show(
    ui: &mut egui::Ui,
    store: &SubscriptionList,
    selected_feed: Option<&str>,
    selected_item_id: &mut Option<String>,
    focus_tag: &mut Option<u8>,
    article_list_tag: u8,
) {
    let items = store.items(selected_feed);

    if items.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.label("No articles.");
        });
        return;
    }

    let mut list_clicked = false;
    egui::ScrollArea::vertical().show(ui, |ui| {
        for item in items {
            let is_selected = selected_item_id.as_deref() == Some(item.id.as_str());
            let date_str = item
                .published
                .as_ref()
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| "?".to_string());
            let label = format!("{}  {}", date_str, item.title);
            let resp = ui.selectable_label(is_selected, label);
            if resp.clicked() {
                *selected_item_id = Some(item.id.clone());
                list_clicked = true;
            }
        }
    });
    if list_clicked {
        *focus_tag = Some(article_list_tag);
    }
}
