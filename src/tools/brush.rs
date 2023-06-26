use egui::{Context, InputState};

use crate::{math, pixel_buffer::PixelBuffer, ui::screen_to_image_coords, TrametesApp};

fn apply_brush(pixels: &mut PixelBuffer, brush: &BrushSettings, pos: (f32, f32), color: &[u8; 4]) {
    let x1 = (pos.0 - brush.diameter / 2.0)
        .clamp(0.0, pixels.width as f32 - 1.0)
        .floor() as usize;
    let y1 = (pos.1 - brush.diameter / 2.0)
        .clamp(0.0, pixels.height as f32 - 1.0)
        .floor() as usize;
    let x2 = (pos.0 + brush.diameter / 2.0)
        .clamp(0.0, pixels.width as f32 - 1.0)
        .ceil() as usize;
    let y2 = (pos.1 + brush.diameter / 2.0)
        .clamp(0.0, pixels.height as f32 - 1.0)
        .ceil() as usize;
    let width = x2 - x1;
    let height = y2 - y1;
    for ((r, g, b, _a), (col, row)) in pixels.iter_block_mut(x1, y1, width, height) {
        // The pixel is a 1x1 rectangle, so the percentage of it that is covered
        // by a circle is just the area of intersection divided by 1 pixel
        // squared
        let percent_of_pixel_covered = math::square_circle_intersection(
            pos,
            brush.diameter / 2.0,
            (col as f32 + 0.5, row as f32 + 0.5),
            1.0,
        ) / 1.0;

        // TODO account for alpha
        assert_eq!(color[3], 255);

        *r = math::lerp(percent_of_pixel_covered, *r as f32, color[0] as f32).round() as u8;
        *g = math::lerp(percent_of_pixel_covered, *g as f32, color[1] as f32).round() as u8;
        *b = math::lerp(percent_of_pixel_covered, *b as f32, color[2] as f32).round() as u8;
    }
}

pub fn handle_input(input: &InputState, app: &mut TrametesApp, ctx: &Context) {
    if let Some(pos) = input.pointer.interact_pos() {
        if input.pointer.any_down() {
            let pixel_pos = screen_to_image_coords(
                pos,
                &app.image_relative_pos,
                (app.image.width as f32, app.image.height as f32),
                ctx.available_rect(),
            );

            apply_brush(
                &mut app.image,
                &app.tools.brush,
                pixel_pos.into(),
                &app.colors.primary,
            )
        }
    }
}

#[derive(Debug)]
pub struct BrushSettings {
    /// The diameter of the brush, in pixels
    diameter: f32,
}

impl Default for BrushSettings {
    fn default() -> Self {
        Self { diameter: 20.0 }
    }
}
