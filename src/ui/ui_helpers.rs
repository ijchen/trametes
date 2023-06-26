use egui::{pos2, Pos2, Rect};
use native_dialog::{MessageDialog, MessageType};

use crate::app::ImageTransformations;

/// Makes a Rect with given (x, y) (top left corner) and width x height
pub fn rect(x: f32, y: f32, width: f32, height: f32) -> Rect {
    Rect::from_min_max(
        Pos2 { x, y },
        Pos2 {
            x: x + width,
            y: y + height,
        },
    )
}

/// Converts image coordinates to screen coordinates
///
/// Image coordinates range from (0, 0) at the top-left of the image to (width,
/// height) of the image (in pixels)
///
/// Screen coordinates range from (0, 0) at the top-left of the window to
/// (width, height) of the screen (in egui "points", which may be multiple
/// physical pixels)
///
/// # Arguments
/// - `image_pos` - the coordinates to transform from image to screen
/// - `image_transformations` - the transformations applied to the image
/// - `image_size` - the dimensions (in pixels) of the image
/// - `image_panel_bounds` - the bounding box (in egui "points") of the panel in
/// the UI that contains the image (excluding the menu bar, control panel, etc.)
pub fn image_to_screen_coords(
    image_pos: Pos2,
    image_transformations: &ImageTransformations,
    image_size: (f32, f32),
    image_panel_bounds: Rect,
) -> Pos2 {
    let base_screen_x = image_panel_bounds.width() / 2.0 + image_panel_bounds.min.x;
    let base_screen_y = image_panel_bounds.height() / 2.0 + image_panel_bounds.min.y;

    let offset_screen_x = base_screen_x + image_transformations.x_translation;
    let offset_screen_y = base_screen_y + image_transformations.y_translation;

    let scaled_image_width = image_size.0 * image_transformations.scale;
    let scaled_image_height = image_size.1 * image_transformations.scale;

    let image_screen_x = offset_screen_x - scaled_image_width / 2.0;
    let image_screen_y = offset_screen_y - scaled_image_height / 2.0;
    let image_screen_width = scaled_image_width;
    let image_screen_height = scaled_image_height;

    let frac_across_width = image_pos.x / image_size.0;
    let frac_across_height = image_pos.y / image_size.1;

    let screen_x = image_screen_x + image_screen_width * frac_across_width;
    let screen_y = image_screen_y + image_screen_height * frac_across_height;

    pos2(screen_x, screen_y)
}

/// Converts screen coordinates to image coordinates
///
/// Image coordinates range from (0, 0) at the top-left of the image to (width,
/// height) of the image (in pixels)
///
/// Screen coordinates range from (0, 0) at the top-left of the window to
/// (width, height) of the screen (in egui "points", which may be multiple
/// physical pixels)
///
/// # Arguments
/// - `screen_pos` - the coordinates to transform from screen to image
/// - `image_transformations` - the transformations applied to the image
/// - `image_size` - the dimensions (in pixels) of the image
/// - `image_panel_bounds` - the bounding box (in egui "points") of the panel in
/// the UI that contains the image (excluding the menu bar, control panel, etc.)
pub fn screen_to_image_coords(
    screen_pos: Pos2,
    image_transformations: &ImageTransformations,
    image_size: (f32, f32),
    image_panel_bounds: Rect,
) -> Pos2 {
    let base_screen_x = image_panel_bounds.width() / 2.0 + image_panel_bounds.min.x;
    let base_screen_y = image_panel_bounds.height() / 2.0 + image_panel_bounds.min.y;

    let offset_screen_x = base_screen_x + image_transformations.x_translation;
    let offset_screen_y = base_screen_y + image_transformations.y_translation;

    let scaled_image_width = image_size.0 * image_transformations.scale;
    let scaled_image_height = image_size.1 * image_transformations.scale;

    let image_screen_x = offset_screen_x - scaled_image_width / 2.0;
    let image_screen_y = offset_screen_y - scaled_image_height / 2.0;
    let image_screen_width = scaled_image_width;
    let image_screen_height = scaled_image_height;

    let dist_across_width = screen_pos.x - image_screen_x;
    let dist_across_height = screen_pos.y - image_screen_y;
    let frac_across_width = dist_across_width / image_screen_width;
    let frac_across_height = dist_across_height / image_screen_height;

    let image_x = image_size.0 * frac_across_width;
    let image_y = image_size.1 * frac_across_height;

    pos2(image_x, image_y)
}

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
