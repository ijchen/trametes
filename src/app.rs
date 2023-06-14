use eframe::{App, CreationContext, Frame};

use crate::{tools::ToolState, ui::draw_ui};

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
pub struct PixelBuffer {
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Default for PixelBuffer {
    fn default() -> Self {
        let width = 800;
        let height = 600;

        const WHITE: [u8; 4] = [255, 255, 255, 255];

        let pixels: Vec<u8> = std::iter::repeat(WHITE)
            .take(width * height)
            .flatten()
            .collect();

        Self {
            pixels,
            width,
            height,
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

/// The persistant state of an instance of Trametes
#[derive(Debug)]
pub struct TrametesApp {
    pub(crate) windows: VisibleWindows,
    pub(crate) image: PixelBuffer,
    pub(crate) image_relative_pos: ImageTransformations,
    pub(crate) tools: ToolState,
}

impl Default for TrametesApp {
    fn default() -> Self {
        Self {
            windows: Default::default(),
            image: Default::default(),
            image_relative_pos: Default::default(),
            tools: Default::default(),
        }
    }
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
