mod controls_panel;
mod main_canvas;
mod menu_bar;
mod ui_helpers;
mod windows;

use crate::TrametesApp;
use eframe::Frame;
use egui::{Context, FontFamily, FontId, TextStyle};

pub use ui_helpers::{image_to_screen_coords, message_popup, rect, screen_to_image_coords};

/// Makes basic and global style changes to the given context
fn set_style(ctx: &egui::Context, frame: &mut Frame) {
    // Set the default text style to be slightly larger
    // TODO is there a cleaner way to do this?
    let mut style = (*ctx.style()).clone();
    let font_size = frame.info().window_info.size.min_elem() / 50.0;
    style.text_styles.insert(
        TextStyle::Name("body_large".into()),
        FontId::new(font_size, FontFamily::Proportional),
    );
    style.override_text_style = Some(TextStyle::Name("body_large".into()));
    ctx.set_style(style);
}

pub fn draw_ui(app: &mut TrametesApp, ctx: &Context, frame: &mut Frame) {
    // Makes basic and global style changes
    set_style(ctx, frame);

    // Top menu bar
    menu_bar::make_top_menu_bar(app, ctx, frame);

    // Top controls panel
    controls_panel::make_controls_panel(app, ctx, frame);

    // The main canvas panel
    main_canvas::make_main_panel(app, ctx, frame);
}
