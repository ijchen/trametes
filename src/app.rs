use std::path::PathBuf;

use eframe::{App, CreationContext, Frame};

use crate::{pixel_buffer::PixelBuffer, tools::ToolState, ui::draw_ui};

#[derive(Debug)]
pub struct VisibleWindows {
    pub tools: bool,
    pub colors: bool,
    pub history: bool,
    pub layers: bool,
}

impl Default for VisibleWindows {
    fn default() -> Self {
        Self {
            tools: true,
            colors: true,
            history: true,
            layers: true,
        }
    }
}

#[derive(Debug)]
pub struct ImageTransformations {
    pub x_translation: f32,
    pub y_translation: f32,
    pub scale: f32,
}

impl Default for ImageTransformations {
    fn default() -> Self {
        Self {
            x_translation: 0.0,
            y_translation: 0.0,
            scale: 1.0,
        }
    }
}

#[derive(Debug)]
pub struct Colors {
    pub primary: [u8; 4],
    pub secondary: [u8; 4],
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            primary: [0, 0, 0, 255],
            secondary: [255, 255, 255, 255],
        }
    }
}

/// The persistant state of an instance of Trametes
#[derive(Debug, Default)]
pub struct TrametesApp {
    /// Keeps track of which draggable windows are currently visible
    pub(crate) windows: VisibleWindows,

    /// A buffer containing the actual pixels of the image
    pub(crate) image: PixelBuffer,

    /// The file path of the image we're editing, if it came from a file
    pub(crate) path: Option<PathBuf>,

    /// The relative visual transformations applied to the image (zooming,
    /// panning, etc.)
    pub(crate) image_relative_pos: ImageTransformations,

    /// The state of the tools (pan, brush, etc.)
    pub(crate) tools: ToolState,

    /// The primary and secondary colors
    pub(crate) colors: Colors,
}

impl TrametesApp {
    /// Called once before the first frame.
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl App for TrametesApp {
    /// Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        draw_ui(self, ctx, frame);
    }
}
