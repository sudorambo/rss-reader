//! Pretty text formatting for article content (HTML to terminal-friendly).
//!
//! Converts HTML to terminal-friendly text: wraps to width, preserves headings,
//! paragraphs, lists, and makes links identifiable (URL or inline reference).

/// Format article body for terminal: strip/reduce HTML, preserve structure (headings, paragraphs, links).
pub fn format_article(html: Option<&str>, width: usize) -> String {
    let width = width.max(40);
    match html {
        None | Some("") => "No content.".to_string(),
        Some(s) => {
            let text = html2text::from_read(s.as_bytes(), width);
            let trimmed = text.trim();
            if trimmed.is_empty() {
                "No content.".to_string()
            } else {
                trimmed.to_string()
            }
        }
    }
}
