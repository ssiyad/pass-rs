use cli_clipboard::{ClipboardContext, ClipboardProvider};

/// Copy the given content to the clipboard
///
/// * `content`:
pub fn copy(content: &str) {
    ClipboardContext::new()
        .expect("Failed to create clipboard context")
        .set_contents(content.to_string())
        .expect("Failed to set clipboard contents");
}
