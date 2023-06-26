use eframe::Frame;
use egui::{Context, Window};

use crate::{tools::Tool, TrametesApp};

use super::rect;

/// Makes the draggable windows (Tools, Colors, History, Layers)
pub fn make_draggable_windows(app: &mut TrametesApp, ctx: &Context, frame: &mut Frame) {
    let width = frame.info().window_info.size.x;
    let height = frame.info().window_info.size.y;

    // The Tools window
    Window::new("Tools")
        .resizable(true)
        .default_rect(rect(0.0, 0.0, width * 0.025, height * 0.33))
        .open(&mut app.windows.tools)
        .show(ctx, |ui| {
            ui.radio_value(
                &mut app.tools.current_tool,
                Tool::Pan,
                Tool::Pan.to_string(),
            );
            ui.radio_value(
                &mut app.tools.current_tool,
                Tool::Brush,
                Tool::Brush.to_string(),
            );

            // Allow filling extra room with empty space (prevents automatic
            // shrinking after resizing)
            ui.allocate_space(ui.available_size());
        });

    // The Colors window
    Window::new("Colors")
        .resizable(true)
        .default_rect(rect(0.0, 9999.0, width * 0.12, width * 0.12)) // TODO not just hardcode 9999
        .open(&mut app.windows.colors)
        .show(ctx, |ui| {
            // TODO Make it so the color wheel/editor is always shown, and there
            // are two overlapping selectable rectangles for selecting the
            // primary and secondary color
            ui.color_edit_button_srgba_unmultiplied(&mut app.colors.primary);
            ui.color_edit_button_srgba_unmultiplied(&mut app.colors.secondary);

            // Allow filling extra room with empty space (prevents automatic
            // shrinking after resizing)
            ui.allocate_space(ui.available_size());
        });

    // The Workspaces window
    Window::new("Workspaces")
        .resizable(true)
        .default_rect(rect(9999.0, 0.0, width * 0.12, height * 0.30)) // TODO not just hardcode 9999
        .open(&mut app.windows.history)
        .show(ctx, |ui| {
            // TODO put stuff here

            // Allow filling extra room with empty space (prevents automatic
            // shrinking after resizing)
            ui.allocate_space(ui.available_size());
        });

    // The Layers window
    Window::new("Layers")
        .resizable(true)
        .default_rect(rect(9999.0, 9999.0, width * 0.12, width * 0.12)) // TODO not just hardcode 9999
        .open(&mut app.windows.layers)
        .show(ctx, |ui| {
            // TODO put stuff here

            // Allow filling extra room with empty space (prevents automatic
            // shrinking after resizing)
            ui.allocate_space(ui.available_size());
        });
}
