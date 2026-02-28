//! Enclosure handling: list, open URL in default app, or download to file.

use crate::feed::MediaEnclosure;
use crate::Error;
use std::path::Path;

/// Open enclosure URL in the system default app (browser, player, etc.).
pub fn open_enclosure(enclosure: &MediaEnclosure) -> Result<(), Error> {
    open::that(&enclosure.url).map_err(|e| Error::Store(format!("open failed: {}", e)))
}

/// Download enclosure to a file in `dest_dir` (or current dir if None).
/// Returns the path of the downloaded file.
pub fn download_enclosure(
    enclosure: &MediaEnclosure,
    dest_dir: Option<&Path>,
) -> Result<std::path::PathBuf, Error> {
    let dir = dest_dir.unwrap_or(Path::new("."));
    let filename = enclosure
        .url
        .rsplit('/')
        .next()
        .unwrap_or("enclosure")
        .split('?')
        .next()
        .unwrap_or("enclosure");
    let path = dir.join(filename);
    let response = reqwest::blocking::get(&enclosure.url)?;
    let bytes = response.bytes()?;
    std::fs::write(&path, &bytes).map_err(Error::Io)?;
    Ok(path)
}

/// Open enclosure URL in external app or download to path (per spec: open or download).
pub fn open_or_download_enclosure(
    enclosure: &MediaEnclosure,
    dest_dir: Option<&Path>,
) -> Result<(), Error> {
    if dest_dir.is_some() {
        download_enclosure(enclosure, dest_dir).map(|_| ())
    } else {
        open_enclosure(enclosure)
    }
}
