//! eframe app: window, run loop, load store, main layout.

use eframe::egui;
use std::path::PathBuf;
use std::sync::mpsc;

use super::views::{add_feed, article_detail, article_list, feed_list};
use crate::fetch::fetch_feed;
use crate::SubscriptionList;
use crate::{Feed, FeedItem};

/// Channel result for add-feed background fetch (avoids type_complexity in struct).
type AddFeedReceiver = mpsc::Receiver<Result<(Feed, Vec<FeedItem>), crate::Error>>;
/// Channel result for refresh background fetch (avoids type_complexity in struct).
type RefreshReceiver = mpsc::Receiver<(Vec<(Feed, Vec<FeedItem>)>, Option<String>)>;

/// Focus tag for arrow-key navigation: 0 = feed list, 1 = article list (FR-010).
const FOCUS_FEED_LIST: u8 = 0;
const FOCUS_ARTICLE_LIST: u8 = 1;

/// Run the GUI. Load store and start eframe.
pub fn run(config_path: PathBuf) -> crate::Result<()> {
    let store = SubscriptionList::load(config_path.as_path())?;
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "RSS Reader",
        options,
        Box::new(move |cc| Ok(Box::new(App::new(cc, store, config_path)))),
    )
    .map_err(|e| crate::Error::Store(e.to_string()))
}

struct App {
    store: SubscriptionList,
    config_path: PathBuf,
    selected_feed: Option<String>,
    selected_item_id: Option<String>,
    add_feed_dialog_open: bool,
    add_feed_url: String,
    add_feed_error: Option<String>,
    add_feed_loading: bool,
    add_feed_pending: Option<AddFeedReceiver>,
    loading: bool,
    refresh_pending: Option<RefreshReceiver>,
    last_error: Option<String>,
    focused_panel: Option<u8>,
}

impl App {
    fn new(
        _cc: &eframe::CreationContext<'_>,
        store: SubscriptionList,
        config_path: PathBuf,
    ) -> Self {
        Self {
            store,
            config_path,
            selected_feed: None,
            selected_item_id: None,
            add_feed_dialog_open: false,
            add_feed_url: String::new(),
            add_feed_error: None,
            add_feed_loading: false,
            add_feed_pending: None,
            loading: false,
            refresh_pending: None,
            last_error: None,
            focused_panel: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Keyboard (FR-010): Tab/Shift+Tab follow widget order (feeds → articles → detail → enclosure buttons).
        // Arrow keys in feed/article list when that list was last clicked; Enter in add-feed dialog; Escape cancels dialog.
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Add feed").clicked() {
                    self.add_feed_dialog_open = true;
                }
                if let Some(ref url) = self.selected_feed {
                    if ui.button("Remove feed").clicked() && self.store.remove_feed(url) {
                        let _ = self.store.save(self.config_path.as_path());
                        self.selected_feed = None;
                        self.selected_item_id = None;
                    }
                }
                if ui.button("Refresh").clicked() && !self.loading {
                    let urls: Vec<String> = self
                        .store
                        .feeds
                        .iter()
                        .map(|f| f.url.clone())
                        .filter(|u| self.selected_feed.as_deref().map_or(true, |f| u == f))
                        .collect();
                    if urls.is_empty() {
                        self.last_error = Some("No feeds to refresh.".to_string());
                    } else {
                        self.loading = true;
                        let (tx, rx) = mpsc::channel();
                        std::thread::spawn(move || {
                            let mut updates = vec![];
                            let mut err_msgs = vec![];
                            for url in urls {
                                match fetch_feed(&url) {
                                    Ok((feed, items)) => updates.push((feed, items)),
                                    Err(e) => err_msgs.push(e.to_string()),
                                }
                            }
                            let err = if err_msgs.is_empty() {
                                None
                            } else {
                                Some(err_msgs.join("; "))
                            };
                            let _ = tx.send((updates, err));
                        });
                        self.refresh_pending = Some(rx);
                    }
                }
            });
        });

        // Poll pending add-feed result (T017)
        let rx_opt = self.add_feed_pending.take();
        if let Some(rx) = rx_opt {
            match rx.try_recv() {
                Ok(Ok((feed, items))) => {
                    self.store.add_feed(feed, items);
                    let _ = self.store.save(self.config_path.as_path());
                    self.add_feed_dialog_open = false;
                    self.add_feed_loading = false;
                    self.add_feed_url.clear();
                    self.add_feed_error = None;
                }
                Ok(Err(e)) => {
                    self.add_feed_error = Some(e.to_string());
                    self.add_feed_loading = false;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.add_feed_pending = Some(rx);
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.add_feed_loading = false;
                }
            }
        }

        // Poll pending refresh result (T020)
        let refresh_rx = self.refresh_pending.take();
        if let Some(rx) = refresh_rx {
            match rx.try_recv() {
                Ok((updates, err_msg)) => {
                    for (feed, items) in updates {
                        self.store.add_feed(feed, items);
                    }
                    let _ = self.store.save(self.config_path.as_path());
                    if let Some(msg) = err_msg {
                        self.last_error = Some(format!("Refresh: {}", msg));
                    }
                    self.loading = false;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.refresh_pending = Some(rx);
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.loading = false;
                }
            }
        }

        if self.add_feed_dialog_open {
            let mut close_dialog = false;
            egui::Window::new("Add feed")
                .collapsible(false)
                .resizable(false)
                .open(&mut self.add_feed_dialog_open)
                .show(ctx, |ui| {
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        close_dialog = true;
                    }
                    if let Some(action) = add_feed::show(
                        ui,
                        &mut self.add_feed_url,
                        &mut self.add_feed_error,
                        self.add_feed_loading,
                    ) {
                        match action {
                            add_feed::AddFeedAction::Submit(url) => {
                                self.add_feed_loading = true;
                                let (tx, rx) = mpsc::channel();
                                std::thread::spawn(move || {
                                    let _ = tx.send(fetch_feed(&url));
                                });
                                self.add_feed_pending = Some(rx);
                            }
                            add_feed::AddFeedAction::Cancel => close_dialog = true,
                        }
                    }
                });
            if close_dialog {
                self.add_feed_dialog_open = false;
                self.add_feed_url.clear();
                self.add_feed_error = None;
            }
        }

        egui::SidePanel::left("feeds")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                feed_list::show(
                    ui,
                    &self.store,
                    &mut self.selected_feed,
                    &mut self.add_feed_dialog_open,
                    &mut self.focused_panel,
                    FOCUS_FEED_LIST,
                );
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            super::widgets::show_error_banner(ui, &mut self.last_error);
            if self.loading {
                super::widgets::show_loading(ui);
            }
            // Use full remaining rect so list/detail get full height (horizontal gives one row otherwise).
            let rect = ui.available_rect_before_wrap();
            let list_width = 280.0_f32.min(rect.width() * 0.35);
            let detail_width = rect.width() - list_width;
            let full_height = rect.height();
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(list_width, full_height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            article_list::show(
                                ui,
                                &self.store,
                                self.selected_feed.as_deref(),
                                &mut self.selected_item_id,
                                &mut self.focused_panel,
                                FOCUS_ARTICLE_LIST,
                            );
                        });
                    },
                );
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(detail_width, full_height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        article_detail::show(
                            ui,
                            &self.store,
                            self.selected_item_id.as_deref(),
                            self.selected_feed.as_deref(),
                        );
                    },
                );
            });
        });

        // Arrow-key navigation (FR-010)
        if !self.add_feed_dialog_open {
            let (arrow_down, arrow_up) = ctx.input(|i| {
                (
                    i.key_pressed(egui::Key::ArrowDown),
                    i.key_pressed(egui::Key::ArrowUp),
                )
            });
            if self.focused_panel == Some(FOCUS_FEED_LIST) && (arrow_down || arrow_up) {
                let urls: Vec<Option<String>> = std::iter::once(None)
                    .chain(self.store.feeds.iter().map(|f| Some(f.url.clone())))
                    .collect();
                let idx = urls
                    .iter()
                    .position(|u| u.as_deref() == self.selected_feed.as_deref())
                    .unwrap_or(0);
                let new_idx = if arrow_down {
                    (idx + 1).min(urls.len().saturating_sub(1))
                } else {
                    idx.saturating_sub(1)
                };
                self.selected_feed = urls.get(new_idx).and_then(|o| o.clone());
            }
            if self.focused_panel == Some(FOCUS_ARTICLE_LIST) && (arrow_down || arrow_up) {
                let items = self.store.items(self.selected_feed.as_deref());
                let ids: Vec<&str> = items.iter().map(|i| i.id.as_str()).collect();
                let idx = self
                    .selected_item_id
                    .as_deref()
                    .and_then(|id| ids.iter().position(|&i| i == id))
                    .unwrap_or(0);
                let new_idx = if arrow_down {
                    (idx + 1).min(ids.len().saturating_sub(1))
                } else {
                    idx.saturating_sub(1)
                };
                self.selected_item_id = ids.get(new_idx).map(|s| (*s).to_string());
            }
        }
    }
}
