use std::path::{Path, PathBuf};

use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer};
use native_dialog::{FileDialog, MessageDialog, MessageType};

use crate::{
    app::{ImageTransformations, PixelBuffer},
    TrametesApp,
};

/// Displays a popup message to the user
// TODO this probably doesn't work on web, need web-specific workaround?
pub fn message_popup(msg: &str, msg_type: MessageType) {
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
fn get_image_path_to_open() -> Option<PathBuf> {
    match FileDialog::new().show_open_single_file() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("failed to get an image file path from the user: {err:?}");
            message_popup("Failed to prompt for a file to open", MessageType::Error);
            None
        }
    }
}

/// Reads an image from a file path
fn read_image_from_file(path: &Path) -> Option<((u32, u32), Vec<u8>)> {
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

/// Prompts the user for an image to open, then opens it
pub fn command_open(app: &mut TrametesApp) {
    match get_image_path_to_open() {
        Some(path) => {
            match read_image_from_file(&path) {
                Some(((width, height), pixels)) => {
                    app.image = PixelBuffer {
                        pixels,
                        width: width as usize,
                        height: height as usize,
                    };
                    app.image_relative_pos = ImageTransformations::default();
                    app.path = Some(path);
                }
                None => {
                    eprintln!("failed to read image from file path: {path:?}");
                    message_popup("Failed to read file", MessageType::Error);
                }
            };
        }
        None => {
            // The user likely hit "cancel", do nothing and
            // carry on
        }
    }
}

/// Prompts the user for a path to save an image to, returning the path (or None
/// if the user did not provide one)
fn get_image_path_to_save_as() -> Option<PathBuf> {
    match FileDialog::new().show_save_single_file() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("failed to get an image file path from the user: {err:?}");
            message_popup("Failed to prompt for a file name", MessageType::Error);
            None
        }
    }
}

/// Saves an image to a file path, displaying an error to the user if saving
/// fails
fn save_image_to_file(path: &Path, image: &PixelBuffer) {
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
        message_popup("Failed to save file", MessageType::Error);
    }
}

/// Prompts the user for a file path to save the current image to, then saves it
pub fn command_save_as(app: &mut TrametesApp) {
    match get_image_path_to_save_as() {
        Some(path) => {
            save_image_to_file(&path, &app.image);
            app.path = Some(path);
        }
        None => {
            // The user likely hit "cancel", do nothing and
            // carry on
        }
    }
}

/// Saves the current image to the file path it came from, or prompts the user
/// for a file path if the current image didn't come from a file, then saves the
/// image to that path
pub fn command_save(app: &mut TrametesApp) {
    match &app.path {
        Some(path) => save_image_to_file(&path, &app.image),
        None => command_save_as(app),
    }
}
