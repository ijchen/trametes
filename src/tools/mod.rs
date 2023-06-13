mod brush;
mod pan;

use egui::{Context, InputState};

use crate::TrametesApp;

/// A "tool" usable in the editor (brush, eraser, pan, shape)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    /// Visually move the image around the window
    #[default]
    Pan,

    /// Draw with a generic circular "brush-like" shape
    Brush,
}

impl Tool {
    /// Handles an input event (usually doing whatever the tool is meant to do)
    pub fn handle_input(&self, input: &InputState, app: &mut TrametesApp, ctx: &Context) {
        use Tool::*;
        match self {
            Pan => pan::handle_input(input, app, ctx),
            Brush => brush::handle_input(input, app, ctx),
        }
    }
}
