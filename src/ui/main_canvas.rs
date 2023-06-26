use eframe::Frame;
use egui::{
    pos2, warn_if_debug_build, CentralPanel, Color32, ColorImage, Context, ImageData, Pos2, Rect,
    TextureFilter, TextureOptions, Ui,
};

use crate::{app::ImageTransformations, TrametesApp};

use super::{image_to_screen_coords, screen_to_image_coords, windows};

fn zoom_image(zoom_delta: f32, zoom_origin: Pos2, app: &mut TrametesApp, panel_rect: Rect) {
    // TODO do we want to do an epsilon comparison here? I feel like
    // it's reasonable to expect *exactly* 1.0, but maybe not. Idk
    if zoom_delta != 1.0 {
        let original_scale = app.image_relative_pos.scale;

        // Adjust the scale
        app.image_relative_pos.scale *= zoom_delta;
        let min_scale = 0.5
            * f32::min(
                panel_rect.width() / app.image.width as f32,
                panel_rect.height() / app.image.height as f32,
            );
        let max_scale = f32::min(panel_rect.width() / 2.0, panel_rect.height() / 2.0);
        app.image_relative_pos.scale = app.image_relative_pos.scale.clamp(min_scale, max_scale);

        // Adjust the x and y translation so the cursor's location
        // relative to the image is unchanged
        let original_image_pos = screen_to_image_coords(
            zoom_origin,
            &ImageTransformations {
                scale: original_scale,
                ..app.image_relative_pos
            },
            (app.image.width as f32, app.image.height as f32),
            panel_rect,
        );
        let new_image_pos = screen_to_image_coords(
            zoom_origin,
            &app.image_relative_pos,
            (app.image.width as f32, app.image.height as f32),
            panel_rect,
        );
        let image_delta_x = original_image_pos.x - new_image_pos.x;
        let image_delta_y = original_image_pos.y - new_image_pos.y;
        let screen_delta_x = image_delta_x * app.image_relative_pos.scale;
        let screen_delta_y = image_delta_y * app.image_relative_pos.scale;
        app.image_relative_pos.x_translation -= screen_delta_x;
        app.image_relative_pos.y_translation -= screen_delta_y;
    }
}

fn clamp_image_to_bounds(app: &mut TrametesApp, panel_rect: Rect) {
    let width = app.image.width as f32 * app.image_relative_pos.scale;
    let height = app.image.height as f32 * app.image_relative_pos.scale;
    let margin = 0.25;
    let min_x = panel_rect.width() * margin - (panel_rect.width() + width) / 2.0;
    let max_x = panel_rect.width() * (1.0 - margin) - (panel_rect.width() - width) / 2.0;
    let min_y = panel_rect.height() * margin - (panel_rect.height() + height) / 2.0;
    let max_y = panel_rect.height() * (1.0 - margin) - (panel_rect.height() - height) / 2.0;
    app.image_relative_pos.x_translation = app.image_relative_pos.x_translation.clamp(min_x, max_x);
    app.image_relative_pos.y_translation = app.image_relative_pos.y_translation.clamp(min_y, max_y);
}

/// Makes the actual image itself
fn make_image(app: &mut TrametesApp, ui: &mut Ui) {
    // Create a texture for the image
    // TODO do we *really* need to recreate a new texture each time?
    let image =
        ColorImage::from_rgba_unmultiplied([app.image.width, app.image.height], &app.image.pixels);
    let image_data = ImageData::Color(image);
    let texture = ui.ctx().load_texture(
        "main image",
        image_data,
        TextureOptions {
            magnification: TextureFilter::Nearest,
            minification: TextureFilter::Linear,
        },
    );

    // Calculate the transformed screen rect to draw the image in
    let pos = Rect::from_min_max(
        image_to_screen_coords(
            pos2(0.0, 0.0),
            &app.image_relative_pos,
            (app.image.width as f32, app.image.height as f32),
            ui.ctx().available_rect(),
        ),
        image_to_screen_coords(
            pos2(app.image.width as f32, app.image.height as f32),
            &app.image_relative_pos,
            (app.image.width as f32, app.image.height as f32),
            ui.ctx().available_rect(),
        ),
    );

    // Draw the image (clipped so it doesn't cover the UI)
    let painter = ui.painter_at(ui.available_rect_before_wrap());
    painter.image(
        (&texture).into(),
        pos,
        Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), // No transform
        Color32::WHITE,                                     // No tint
    );
}

/// Makes the "main panel" (the large central area with the editable image)
pub fn make_main_panel(app: &mut TrametesApp, ctx: &Context, frame: &mut Frame) {
    // The panel frame is used for adjusting the style of the panel
    let panel_frame = egui::containers::Frame {
        fill: Color32::from_rgb(50, 50, 50),
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        // Handle user inputs
        // TODO separate this out - I don't think it belongs in UI code
        ui.input(|input| {
            let panel_rect = ui.ctx().available_rect();

            // Handle zooming
            let zoom_origin = input
                .pointer
                .interact_pos()
                .filter(|pos| panel_rect.contains(*pos))
                .unwrap_or(panel_rect.center());
            zoom_image(input.zoom_delta(), zoom_origin, app, panel_rect);

            // Let whatever tool is active do its thing
            let current_tool = app.tools.current_tool;
            current_tool.handle_input(input, app, ctx);

            // Ensure the image is in-bounds
            clamp_image_to_bounds(app, panel_rect);
        });

        windows::make_draggable_windows(app, ctx, frame);

        make_image(app, ui);

        warn_if_debug_build(ui);
    });
}
