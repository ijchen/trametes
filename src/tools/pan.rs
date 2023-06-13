use egui::{Context, InputState};

use crate::TrametesApp;

pub fn handle_input(input: &InputState, app: &mut TrametesApp, _ctx: &Context) {
    // TODO not do this janky dt hack to get around
    // is_decidedly_dragging() not handling file -> open well
    if input.pointer.is_decidedly_dragging() && input.unstable_dt < 1.0 {
        app.image_relative_pos.x_translation += input.pointer.delta().x;
        app.image_relative_pos.y_translation += input.pointer.delta().y;
    };
}
