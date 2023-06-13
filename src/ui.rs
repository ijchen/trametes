use eframe::Frame;
use egui::{
    pos2, warn_if_debug_build, CentralPanel, Color32, ColorImage, Context, FontFamily, FontId,
    ImageData, Pos2, Rect, TextStyle, TextureFilter, TextureOptions, TopBottomPanel, Ui, Window,
};
use native_dialog::MessageType;

use crate::{
    app::{ImageTransformations, PixelBuffer},
    fileio::{get_image_path, info_popup, read_image_from_file},
    TrametesApp,
};

/// Makes a Rect with given (x, y) (top left corner) and width x height
fn rect(x: f32, y: f32, width: f32, height: f32) -> Rect {
    Rect::from_min_max(
        Pos2 { x, y },
        Pos2 {
            x: x + width,
            y: y + height,
        },
    )
}

/// Converts image coordinates to screen coordinates
///
/// Image coordinates range from (0, 0) at the top-left of the image to (width,
/// height) of the image (in pixels)
///
/// Screen coordinates range from (0, 0) at the top-left of the window to
/// (width, height) of the screen (in egui "points", which may be multiple
/// physical pixels)
///
/// # Arguments
/// - `image_pos` - the coordinates to transform from image to screen
/// - `image_transformations` - the transformations applied to the image
/// - `image_size` - the dimensions (in pixels) of the image
/// - `image_panel_bounds` - the bounding box (in egui "points") of the panel in
/// the UI that contains the image (excluding the menu bar, control panel, etc.)
pub fn image_to_screen_coords(
    image_pos: Pos2,
    image_transformations: &ImageTransformations,
    image_size: (f32, f32),
    image_panel_bounds: Rect,
) -> Pos2 {
    let base_screen_x =
        (image_panel_bounds.max.x - image_panel_bounds.min.x) / 2.0 + image_panel_bounds.min.x;
    let base_screen_y =
        (image_panel_bounds.max.y - image_panel_bounds.min.y) / 2.0 + image_panel_bounds.min.y;

    let offset_screen_x = base_screen_x + image_transformations.x_translation;
    let offset_screen_y = base_screen_y + image_transformations.y_translation;

    let scaled_image_width = image_size.0 * image_transformations.scale;
    let scaled_image_height = image_size.1 * image_transformations.scale;

    let transformed_image_x = offset_screen_x - scaled_image_width / 2.0;
    let transformed_image_y = offset_screen_y - scaled_image_height / 2.0;
    let transformed_image_width = scaled_image_width;
    let transformed_image_height = scaled_image_height;

    let frac_across_width = image_pos.x / image_size.0;
    let frac_across_height = image_pos.y / image_size.1;

    let screen_x = transformed_image_x + transformed_image_width * frac_across_width;
    let screen_y = transformed_image_y + transformed_image_height * frac_across_height;

    pos2(screen_x, screen_y)
}

/// Makes basic and global style changes to the given context
fn set_style(ctx: &egui::Context, frame: &mut Frame) {
    // Set the default text style to be slightly larger
    // TODO is there a cleaner way to do this?
    let mut style = (*ctx.style()).clone();
    let font_size = frame.info().window_info.size.min_elem() / 50.0;
    style.text_styles.insert(
        TextStyle::Name("body_large".into()),
        FontId::new(font_size, FontFamily::Proportional),
    );
    style.override_text_style = Some(TextStyle::Name("body_large".into()));
    ctx.set_style(style);
}

/// Makes the top menu bar (File, Edit, View, etc.)
fn make_top_menu_bar(app: &mut TrametesApp, ctx: &Context, frame: &mut Frame) {
    TopBottomPanel::top("top_menu_bar_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            // File
            ui.menu_button("File", |ui| {
                if ui.button("New...").clicked() {
                    todo!()
                }

                if ui.button("Open...").clicked() {
                    match get_image_path() {
                        Some(path) => {
                            match read_image_from_file(&path) {
                                Some(((width, height), pixels)) => {
                                    app.image = PixelBuffer {
                                        pixels,
                                        width: width as usize,
                                        height: height as usize,
                                    };

                                    app.image_relative_pos = ImageTransformations::default();
                                }
                                None => {
                                    eprintln!("failed to read image from file path: {path:?}");
                                    info_popup("Failed to read file", MessageType::Error);
                                }
                            };
                        }
                        None => {
                            // The user likely hit "cancel", do nothing and
                            // carry on
                        }
                    }
                }

                ui.menu_button("Open Recent", |ui| {
                    // TODO
                    if ui.button("TODO put stuff here lol").clicked() {
                        todo!()
                    }
                });

                ui.separator();

                if ui.button("Save").clicked() {
                    todo!()
                }

                if ui.button("Save As...").clicked() {
                    todo!()
                }

                ui.separator();

                if ui.button("Settings...").clicked() {
                    todo!()
                }

                // No "Quit" on the web
                if !frame.is_web() {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                }
            });

            // Edit
            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    todo!()
                }

                if ui.button("Redo").clicked() {
                    todo!()
                }

                ui.separator();

                if ui.button("Cut").clicked() {
                    todo!()
                }

                if ui.button("Copy").clicked() {
                    todo!()
                }

                if ui.button("Paste").clicked() {
                    todo!()
                }

                if ui.button("Paste into New Image").clicked() {
                    todo!()
                }
            });
            ui.menu_button("View", |ui| {
                ui.menu_button("Windows", |ui| {
                    ui.checkbox(&mut app.windows.tools, "Tools");
                    ui.checkbox(&mut app.windows.colors, "Colors");
                    ui.checkbox(&mut app.windows.history, "History");
                    ui.checkbox(&mut app.windows.layers, "Layers");
                });
            });

            // Image
            ui.menu_button("Image", |ui| {
                // TODO put stuff here
                ui.label("TODO put stuff here");
            });

            // Layer
            ui.menu_button("Layer", |ui| {
                // TODO put stuff here
                ui.label("TODO put stuff here");
            });

            // Effects
            ui.menu_button("Effects", |ui| {
                // TODO put stuff here
                ui.label("TODO put stuff here");
            });
        });
    });
}

/// Make the controls panel (across the top, just below the menu bar)
fn make_controls_panel(_app: &mut TrametesApp, ctx: &Context, frame: &mut Frame) {
    let top_controls_panel_min_height = frame.info().window_info.size.y * 0.05;
    let top_controls_panel_max_height = frame.info().window_info.size.y * 0.50;
    let top_controls_panel_default_height = frame.info().window_info.size.y * 0.08;
    TopBottomPanel::top("top_controls_panel")
        .resizable(true)
        .min_height(top_controls_panel_min_height)
        .max_height(top_controls_panel_max_height)
        .default_height(top_controls_panel_default_height)
        .show(ctx, |ui| {
            // TODO put stuff here

            // Allow filling extra room with empty space (prevents automatic
            // shrinking after resizing)
            ui.allocate_space(ui.available_size());
        });
}

/// Makes the draggable windows (Tools, Colors, History, Layers)
fn make_draggable_windows(app: &mut TrametesApp, ctx: &Context, frame: &mut Frame) {
    let width = frame.info().window_info.size.x;
    let height = frame.info().window_info.size.y;

    // The Tools window
    Window::new("Tools")
        .resizable(true)
        .default_rect(rect(0.0, 0.0, width * 0.025, height * 0.33))
        .open(&mut app.windows.tools)
        .show(ctx, |ui| {
            // TODO put stuff here

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
            // TODO put stuff here

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
fn make_main_panel(app: &mut TrametesApp, ctx: &Context, frame: &mut Frame) {
    // The panel frame is used for adjusting the style of the panel
    let panel_frame = egui::containers::Frame {
        fill: Color32::from_rgb(50, 50, 50),
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        // Handle user inputs
        ui.input(|input| {
            app.image_relative_pos.scale *= f32::powf(1.01, input.scroll_delta.y);
            let panel_rect = ui.ctx().available_rect();
            let min_scale = 0.5
                * f32::min(
                    panel_rect.width() / app.image.width as f32,
                    panel_rect.height() / app.image.height as f32,
                );
            let max_scale = f32::min(
                panel_rect.width() as f32 / 2.0,
                panel_rect.height() as f32 / 2.0,
            );
            app.image_relative_pos.scale = app.image_relative_pos.scale.clamp(min_scale, max_scale);
            app.image_relative_pos.scale;

            // TODO not do this janky dt hack to get around
            // is_decidedly_dragging() not handling file -> open well
            if input.pointer.is_decidedly_dragging() && input.unstable_dt < 1.0 {
                app.image_relative_pos.x_translation += input.pointer.delta().x;
                app.image_relative_pos.y_translation += input.pointer.delta().y;
            };

            // Ensure the image is in-bounds
            let width = app.image.width as f32 * app.image_relative_pos.scale;
            let height = app.image.height as f32 * app.image_relative_pos.scale;
            let margin = 0.25;
            let min_x = panel_rect.width() * margin - (panel_rect.width() + width) / 2.0;
            let max_x = panel_rect.width() * (1.0 - margin) - (panel_rect.width() - width) / 2.0;
            let min_y = panel_rect.height() * margin - (panel_rect.height() + height) / 2.0;
            let max_y = panel_rect.height() * (1.0 - margin) - (panel_rect.height() - height) / 2.0;
            app.image_relative_pos.x_translation =
                app.image_relative_pos.x_translation.clamp(min_x, max_x);
            app.image_relative_pos.y_translation =
                app.image_relative_pos.y_translation.clamp(min_y, max_y);
        });

        make_draggable_windows(app, ctx, frame);

        make_image(app, ui);

        warn_if_debug_build(ui);
    });
}

pub fn draw_ui(app: &mut TrametesApp, ctx: &Context, frame: &mut Frame) {
    // Makes basic and global style changes
    set_style(ctx, frame);

    // Top menu bar
    make_top_menu_bar(app, ctx, frame);

    // Top controls panel
    make_controls_panel(app, ctx, frame);

    // The main canvas panel
    make_main_panel(app, ctx, frame);
}