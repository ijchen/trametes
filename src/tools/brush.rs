use egui::{Context, InputState};

use crate::{ui::screen_to_image_coords, TrametesApp};

pub fn handle_input(input: &InputState, app: &mut TrametesApp, ctx: &Context) {
    if let Some(pos) = input.pointer.interact_pos() {
        if input.pointer.any_down() {
            let pixel_pos = screen_to_image_coords(
                pos,
                &app.image_relative_pos,
                (app.image.width as f32, app.image.height as f32),
                ctx.available_rect(),
            );

            let pixel_col = (pixel_pos.x - 0.5).round() as isize;
            let pixel_row = (pixel_pos.y - 0.5).round() as isize;
            if (0..app.image.width).contains(&(pixel_col as usize))
                && (0..app.image.height).contains(&(pixel_row as usize))
            {
                let buffer_index = (pixel_col as usize + app.image.width * pixel_row as usize) * 4;
                app.image.pixels[buffer_index + 0] = 0;
                app.image.pixels[buffer_index + 1] = 0;
                app.image.pixels[buffer_index + 2] = 0;
                app.image.pixels[buffer_index + 3] = 255;
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct BrushSettings {}
