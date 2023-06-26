use eframe::Frame;
use egui::{Context, DragValue, TopBottomPanel, Ui};

use crate::{tools::Tool, TrametesApp};

fn pan_controls(_ui: &mut Ui, _app: &mut TrametesApp) {
    // No controls for pan
}

fn brush_controls(ui: &mut Ui, app: &mut TrametesApp) {
    ui.add(
        DragValue::new(&mut app.tools.brush.diameter)
            .clamp_range(0.0..=500.0)
            .prefix("Size: "),
    );
}

/// Make the controls panel (across the top, just below the menu bar)
pub fn make_controls_panel(app: &mut TrametesApp, ctx: &Context, frame: &mut Frame) {
    let top_controls_panel_min_height = frame.info().window_info.size.y * 0.05;
    let top_controls_panel_max_height = frame.info().window_info.size.y * 0.50;
    let top_controls_panel_default_height = frame.info().window_info.size.y * 0.08;
    TopBottomPanel::top("top_controls_panel")
        .resizable(true)
        .min_height(top_controls_panel_min_height)
        .max_height(top_controls_panel_max_height)
        .default_height(top_controls_panel_default_height)
        .show(ctx, |ui| {
            use Tool::*;
            match app.tools.current_tool {
                Pan => pan_controls(ui, app),
                Brush => brush_controls(ui, app),
            }

            // Allow filling extra room with empty space (prevents automatic
            // shrinking after resizing)
            ui.allocate_space(ui.available_size());
        });
}
