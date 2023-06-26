mod brush;
mod pan;

use std::fmt::Display;

use egui::{Context, InputState};

use crate::TrametesApp;

use self::{brush::BrushSettings, pan::PanSettings};

/// A "tool" usable in the editor (brush, eraser, pan, shape)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    /// Visually move the image around the window
    #[default]
    Pan,

    /// Draw with a generic circular "brush-like" shape
    Brush,
}

impl Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pan => "Pan",
                Self::Brush => "Brush",
            }
        )
    }
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

#[derive(Debug, Default)]
pub struct ToolState {
    pub current_tool: Tool,
    pub pan: PanSettings,
    pub brush: BrushSettings,
}
