//! Open or download a media enclosure by item id and index.

use crate::media;
use crate::SubscriptionList;
use std::path::Path;

pub fn run(
    store: &SubscriptionList,
    item_id: &str,
    index: usize,
    download: bool,
    output_dir: Option<&Path>,
) -> crate::Result<()> {
    let item = store
        .get_item(item_id, None)
        .ok_or_else(|| crate::Error::NotFound(format!("item not found: {}", item_id)))?;
    let enclosure = item
        .enclosures
        .get(index)
        .ok_or_else(|| crate::Error::NotFound(format!("enclosure index {} not found", index)))?;
    if download {
        let path = media::download_enclosure(enclosure, output_dir)?;
        println!("Downloaded to {}", path.display());
    } else {
        media::open_enclosure(enclosure)?;
        println!("Opened {}", enclosure.url);
    }
    Ok(())
}
