//! CLI subcommands: add, remove, list-feeds, list-items, show, refresh.

use crate::SubscriptionList;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rss-reader", about = "Full-featured RSS reader CLI")]
pub struct Args {
    #[arg(long, short, default_value = "human")]
    pub output: String,

    #[arg(long, short)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Add {
        url: String,
    },
    Remove {
        url: String,
    },
    ListFeeds,
    ListItems {
        feed: Option<String>,
    },
    Show {
        item_id: String,
    },
    Refresh {
        feed: Option<String>,
    },
    /// Open or download a media enclosure by item id and enclosure index (0-based).
    OpenEnclosure {
        item_id: String,
        index: usize,
        #[arg(long)]
        download: bool,
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rss-reader")
        .join("data.json")
}

/// Whether stdout should be JSON (from --output json).
pub fn output_json(args: &Args) -> bool {
    args.output.to_lowercase() == "json"
}

pub fn run() -> crate::Result<()> {
    let args = Args::parse();
    let json = output_json(&args);
    let path = args.config.unwrap_or_else(config_path);
    let mut store = SubscriptionList::load(path.as_path())?;

    match &args.cmd {
        Command::Add { url } => add::run(&mut store, url, &path, json),
        Command::Remove { url } => remove::run(&mut store, url, &path, json),
        Command::ListFeeds => list_feeds::run(&store, json),
        Command::ListItems { feed } => list_items::run(&store, feed.as_deref(), json),
        Command::Show { item_id } => show::run(&store, item_id, json),
        Command::Refresh { feed } => refresh::run(&mut store, feed.as_deref(), &path, json),
        Command::OpenEnclosure {
            item_id,
            index,
            download,
            output_dir,
        } => open_enclosure::run(&store, item_id, *index, *download, output_dir.as_deref()),
    }
}

pub mod add;
pub mod list_feeds;
pub mod list_items;
pub mod open_enclosure;
pub mod refresh;
pub mod remove;
pub mod show;
