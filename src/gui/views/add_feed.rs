//! Add-feed dialog: URL input, Add/Cancel, validation, loading state (FR-004, FR-009, T019).

use eframe::egui;

/// Action returned by the add-feed dialog.
pub enum AddFeedAction {
    /// User submitted a valid URL (caller should start fetch and show loading).
    Submit(String),
    /// User cancelled (close dialog).
    Cancel,
}

/// Validate URL: non-empty and parseable. Returns error message if invalid.
fn validate_url(url: &str) -> Option<String> {
    let url = url.trim();
    if url.is_empty() {
        return Some("URL cannot be empty.".to_string());
    }
    if url.parse::<url::Url>().is_err() {
        return Some("Invalid URL.".to_string());
    }
    None
}

/// Draw add-feed dialog. Caller handles Submit by starting fetch and showing loading.
/// - `url`: current URL input (mutated by text edit).
/// - `error`: in-dialog error message (mutated; cleared on new input).
/// - `loading`: when true, show loading UI instead of form (caller sets when fetch started).
///
/// Returns Some(action) when user submits valid URL or cancels; None otherwise.
pub fn show(
    ui: &mut egui::Ui,
    url: &mut String,
    error: &mut Option<String>,
    loading: bool,
) -> Option<AddFeedAction> {
    let mut action: Option<AddFeedAction> = None;

    if loading {
        ui.add_space(10.0);
        ui.spinner();
        ui.label("Loading…");
        return None;
    }

    ui.label("Feed URL:");
    let url_edit = egui::TextEdit::singleline(url)
        .hint_text("https://example.com/feed.xml")
        .desired_width(320.0)
        .id(egui::Id::new("add_feed_url"));
    let url_resp = ui.add(url_edit);

    // Enter to submit
    if url_resp.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        if let Some(msg) = validate_url(url) {
            *error = Some(msg);
        } else {
            action = Some(AddFeedAction::Submit(url.trim().to_string()));
        }
    }

    // Clear error when user types
    if url_resp.changed() {
        *error = None;
    }

    if let Some(ref msg) = *error {
        ui.colored_label(egui::Color32::RED, msg);
    }

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        if ui.button("Add").clicked() {
            if let Some(msg) = validate_url(url) {
                *error = Some(msg);
            } else {
                action = Some(AddFeedAction::Submit(url.trim().to_string()));
            }
        }
        if ui.button("Cancel").clicked() {
            action = Some(AddFeedAction::Cancel);
        }
    });

    action
}
