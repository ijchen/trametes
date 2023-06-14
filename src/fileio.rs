use std::path::{Path, PathBuf};

use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer};
use native_dialog::{FileDialog, MessageType};

use crate::{app::PixelBuffer, ui};

/// Prompts the user for an image file to open, returning a path (or None if the
/// user did not provide one)
pub fn get_image_path_to_open() -> Option<PathBuf> {
    match FileDialog::new().show_open_single_file() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("failed to get an image file path from the user: {err:?}");
            ui::message_popup("Failed to prompt for a file to open", MessageType::Error);
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

/// Prompts the user for a path to save an image to, returning the path (or None
/// if the user did not provide one)
pub fn get_image_path_to_save_as() -> Option<PathBuf> {
    match FileDialog::new().show_save_single_file() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("failed to get an image file path from the user: {err:?}");
            ui::message_popup("Failed to prompt for a file name", MessageType::Error);
            None
        }
    }
}

/// Saves an image to a file path, displaying an error to the user if saving
/// fails
pub fn save_image_to_file(path: &Path, image: &PixelBuffer) {
    let img = DynamicImage::ImageRgba8(
        ImageBuffer::from_raw(
            image.width as u32,
            image.height as u32,
            // TODO how can we avoid this clone? It shouldn't be necessary
            image.pixels.clone(),
        )
        .unwrap(),
    );

    if let Err(err) = img.save(path) {
        eprintln!("failed to save image to file: {err:?}");
        ui::message_popup("Failed to save file", MessageType::Error);
    }
}
