use eframe::{App, CreationContext, Frame};
use egui::{Color32, ColorImage, FontFamily, FontId, ImageData, Pos2, Rect, TextStyle, Ui};

#[derive(Debug)]
struct Windows {
    tools: bool,
    colors: bool,
    history: bool,
    layers: bool,
}

impl Default for Windows {
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
struct PixelBuffer {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

impl Default for PixelBuffer {
    fn default() -> Self {
        let width = 800;
        let height = 600;
        // let pixels = vec![u8::MAX; width * height * 4];
        let pixels = [(0, 0, 0), (255, 0, 255)]
            .into_iter()
            .cycle()
            .take(width)
            .chain([(255, 0, 255), (0, 0, 0)].into_iter().cycle().take(width))
            .cycle()
            .take(width * height)
            .map(|(r, g, b)| [r, g, b, 255])
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
struct ImageRelativePos {
    x: f32,
    y: f32,
    scale: f32,
}

impl Default for ImageRelativePos {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale: 1.0,
        }
    }
}

/// The persistant state of an instance of Trametes
#[derive(Debug)]
pub struct TrametesApp {
    windows: Windows,
    image: PixelBuffer,
    image_relative_pos: ImageRelativePos,
}

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

impl Default for TrametesApp {
    fn default() -> Self {
        Self {
            windows: Default::default(),
            image: Default::default(),
            image_relative_pos: Default::default(),
        }
    }
}

impl TrametesApp {
    /// Called once before the first frame.
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Default::default()
    }
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
fn make_top_menu_bar(app: &mut TrametesApp, ctx: &egui::Context, frame: &mut Frame) {
    egui::TopBottomPanel::top("top_menu_bar_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            // File
            ui.menu_button("File", |ui| {
                if ui.button("New...").clicked() {
                    todo!()
                }

                if ui.button("Open...").clicked() {
                    todo!()
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

            // View
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
fn make_controls_panel(_app: &mut TrametesApp, ctx: &egui::Context, frame: &mut Frame) {
    let top_controls_panel_min_height = frame.info().window_info.size.y * 0.05;
    let top_controls_panel_max_height = frame.info().window_info.size.y * 0.50;
    let top_controls_panel_default_height = frame.info().window_info.size.y * 0.08;
    egui::TopBottomPanel::top("top_controls_panel")
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
fn make_draggable_windows(app: &mut TrametesApp, ctx: &egui::Context, frame: &mut Frame) {
    let width = frame.info().window_info.size.x;
    let height = frame.info().window_info.size.y;

    // The Tools window
    egui::Window::new("Tools")
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
    egui::Window::new("Colors")
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
    egui::Window::new("Workspaces")
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
    egui::Window::new("Layers")
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
    // TODO do we *really* need to reallocate a texture each time? I don't think
    // we're supposed to
    let image =
        ColorImage::from_rgba_unmultiplied([app.image.width, app.image.height], &app.image.pixels);
    let image_data = ImageData::Color(image);
    let texture = ui
        .ctx()
        .load_texture("TODO", image_data, Default::default()); // TODO name

    // TODO calculate this in a good way
    let panel_rect = ui.ctx().available_rect();
    let width = app.image.width as f32 * app.image_relative_pos.scale;
    let height = app.image.height as f32 * app.image_relative_pos.scale;
    let pos = rect(
        panel_rect.min.x + (panel_rect.width() - width) / 2.0 + app.image_relative_pos.x,
        panel_rect.min.y + (panel_rect.height() - height) / 2.0 + app.image_relative_pos.y,
        width,
        height,
    );

    egui::Image::new(&texture, texture.size_vec2()).paint_at(ui, pos);
}

/// Makes the "main panel" (the large central area with the editable image)
pub fn make_main_panel(app: &mut TrametesApp, ctx: &egui::Context, frame: &mut Frame) {
    // The panel frame is used for adjusting the style of the panel
    let panel_frame = egui::containers::Frame {
        fill: Color32::from_rgb(50, 50, 50),
        ..Default::default()
    };

    egui::CentralPanel::default()
        .frame(panel_frame)
        .show(ctx, |ui| {
            // TODO temp testing
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
                app.image_relative_pos.scale =
                    app.image_relative_pos.scale.clamp(min_scale, max_scale);
                app.image_relative_pos.scale;

                if input.pointer.is_decidedly_dragging() {
                    app.image_relative_pos.x += input.pointer.delta().x;
                    app.image_relative_pos.y += input.pointer.delta().y;
                };

                // Ensure the image is in-bounds
                let width = app.image.width as f32 * app.image_relative_pos.scale;
                let height = app.image.height as f32 * app.image_relative_pos.scale;
                let margin = 0.25;
                let min_x = panel_rect.width() * margin - (panel_rect.width() + width) / 2.0;
                let max_x =
                    panel_rect.width() * (1.0 - margin) - (panel_rect.width() - width) / 2.0;
                let min_y = panel_rect.height() * margin - (panel_rect.height() + height) / 2.0;
                let max_y =
                    panel_rect.height() * (1.0 - margin) - (panel_rect.height() - height) / 2.0;
                app.image_relative_pos.x = app.image_relative_pos.x.clamp(min_x, max_x);
                app.image_relative_pos.y = app.image_relative_pos.y.clamp(min_y, max_y);
            });

            make_draggable_windows(app, ctx, frame);

            // The image itself
            make_image(app, ui);

            egui::warn_if_debug_build(ui);
        });
}

impl App for TrametesApp {
    /// Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        // Makes basic and global style changes
        set_style(ctx, frame);

        // Top menu bar
        make_top_menu_bar(self, ctx, frame);

        // Top controls panel
        make_controls_panel(self, ctx, frame);

        // The main canvas panel
        make_main_panel(self, ctx, frame);
    }
}
