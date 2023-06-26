use eframe::Frame;
use egui::{Context, TopBottomPanel};

use crate::TrametesApp;

/// Make the controls panel (across the top, just below the menu bar)
pub fn make_controls_panel(_app: &mut TrametesApp, ctx: &Context, frame: &mut Frame) {
    let top_controls_panel_min_height = frame.info().window_info.size.y * 0.05;
    let top_controls_panel_max_height = frame.info().window_info.size.y * 0.50;
    let top_controls_panel_default_height = frame.info().window_info.size.y * 0.08;
    TopBottomPanel::top("top_controls_panel")
        .resizable(true)
        .min_height(top_controls_panel_min_height)
        .max_height(top_controls_panel_max_height)
        .default_height(top_controls_panel_default_height)
        .show(ctx, |ui| {
            // TODO put stuff here

            // Allow filling extra room with empty space (prevents automatic
            // shrinking after resizing)
            ui.allocate_space(ui.available_size());
        });
}
