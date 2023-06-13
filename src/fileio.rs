use std::path::{Path, PathBuf};

use image::{io::Reader, GenericImageView};
use native_dialog::{FileDialog, MessageDialog, MessageType};

/// Displays a popup message to the user
pub fn info_popup(msg: &str, msg_type: MessageType) {
    let title = match msg_type {
        MessageType::Info => "Info",
        MessageType::Warning => "Warning",
        MessageType::Error => "Something went wrong",
    };
    MessageDialog::new()
        .set_type(msg_type)
        .set_title(title)
        .set_text(msg)
        .show_alert()
        .unwrap(); // TODO handle errors here
}

/// Prompts the user for an image file to open, returning a path (or None if the
/// user did not provide one)
pub fn get_image_path() -> Option<PathBuf> {
    match FileDialog::new().show_open_single_file() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("failed to get an image file path from the user: {err:?}");
            info_popup("Failed to prompt for a file to open", MessageType::Error);
            None
        }
    }
}

/// Reads an image from a file path
pub fn read_image_from_file(path: &Path) -> Option<((u32, u32), Vec<u8>)> {
    let img = Reader::open(path)
        .ok()?
        // Guess the encoding format based on the file contents instead of the
        // extension
        .with_guessed_format()
        .ok()?
        .decode()
        .ok()?;

    let (width, height) = img.dimensions();

    let pixels = img
        .pixels()
        .flat_map(|(_, _, pixel)| pixel.0.into_iter())
        .collect();

    Some(((width, height), pixels))
}
