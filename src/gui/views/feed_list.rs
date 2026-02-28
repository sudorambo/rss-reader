//! Feed list view: "All" + subscribed feeds, single selection, arrow keys (FR-001).

use crate::SubscriptionList;
use eframe::egui;

/// Draw feed list; update `selected_feed` on click (None = "All", Some(url) = feed).
/// If no feeds, show empty state and set `open_add_feed` true when "Add feed" is clicked (FR-008).
/// Set `*focus_tag = Some(feed_list_tag)` when user clicks in the list for arrow-key handling.
pub fn show(
    ui: &mut egui::Ui,
    store: &SubscriptionList,
    selected_feed: &mut Option<String>,
    open_add_feed: &mut bool,
    focus_tag: &mut Option<u8>,
    feed_list_tag: u8,
) {
    if store.feeds.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.label("No feeds yet.");
            ui.label("Add a feed to get started.");
            ui.add_space(10.0);
            if ui.button("Add feed").clicked() {
                *open_add_feed = true;
            }
        });
        return;
    }

    let mut list_clicked = false;
    egui::ScrollArea::vertical().show(ui, |ui| {
        // "All" option
        let all_selected = selected_feed.is_none();
        let resp = ui.selectable_label(all_selected, "All");
        if resp.clicked() {
            *selected_feed = None;
            list_clicked = true;
        }

        for feed in &store.feeds {
            let label = feed.title.as_deref().unwrap_or(feed.url.as_str());
            let is_selected = selected_feed.as_deref() == Some(feed.url.as_str());
            let resp = ui.selectable_label(is_selected, label);
            if resp.clicked() {
                *selected_feed = Some(feed.url.clone());
                list_clicked = true;
            }
        }
    });
    if list_clicked {
        *focus_tag = Some(feed_list_tag);
    }
}
