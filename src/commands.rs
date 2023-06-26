use std::borrow::Cow;

use arboard::{Clipboard, ImageData};
use egui::egui_assert;
use native_dialog::MessageType;

use crate::{app::ImageTransformations, fileio, pixel_buffer::PixelBuffer, ui, TrametesApp};

/// Shows a "feature not available" popup box
pub fn todo(feature_name: &str) {
    let msg = format!("Feature '{feature_name}' is not yet available");
    ui::message_popup(&msg, MessageType::Info);
}

/// Creates a new image (erasing anything previously drawn)
pub fn new(app: &mut TrametesApp) {
    // TODO let the user choose the new width and height, and whatever else
    app.image = PixelBuffer::default();
    app.image_relative_pos = ImageTransformations::default();
    app.path = None;
}

/// Prompts the user for an image to open, then opens it
pub fn open(app: &mut TrametesApp) {
    match fileio::get_image_path_to_open() {
        Some(path) => {
            match fileio::read_image_from_file(&path) {
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
                    ui::message_popup("Failed to read file", MessageType::Error);
                }
            };
        }
        None => {
            // The user likely hit "cancel", do nothing and
            // carry on
        }
    }
}

/// Prompts the user for a file path to save the current image to, then saves it
pub fn save_as(app: &mut TrametesApp) {
    match fileio::get_image_path_to_save_as() {
        Some(path) => {
            fileio::save_image_to_file(&path, &app.image);
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
pub fn save(app: &mut TrametesApp) {
    match &app.path {
        Some(path) => fileio::save_image_to_file(&path, &app.image),
        None => save_as(app),
    }
}

/// Copies the selected part of the image into the clipboard, or the entire
/// image if there is no active selection
pub fn copy(app: &mut TrametesApp) {
    // TODO only copy selection if there is an active selection

    // TODO spawn a process to persist with the clipboard even if Trametes is
    // closed (see https://docs.rs/arboard/latest/arboard/trait.SetExtLinux.html#tymethod.wait)
    // (only necessary on linux)

    let mut clipboard = Clipboard::new().unwrap(); // TODO handle errors here

    let image_data = ImageData {
        width: app.image.width,
        height: app.image.height,
        bytes: Cow::from(&app.image.pixels),
    };

    clipboard.set_image(image_data).unwrap(); // TODO handle errors here
}

/// Pastes an image from the clipboard into a new image
pub fn paste_into_new_image(app: &mut TrametesApp) {
    let mut clipboard = Clipboard::new().unwrap(); // TODO handle errors here

    let image = clipboard.get_image().unwrap(); // TODO handle errors here

    let width = image.width;
    let height = image.height;
    let bytes = image.bytes;

    egui_assert!(bytes.len() == width * height * 4);

    app.image.width = image.width;
    app.image.height = image.height;
    app.image.pixels = bytes.to_vec();
    app.image_relative_pos = Default::default();
    app.path = None;
}
