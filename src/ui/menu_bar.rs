use eframe::Frame;
use egui::{Context, TopBottomPanel};

use crate::{commands, TrametesApp};

/// Makes the top menu bar (File, Edit, View, etc.)
pub fn make_top_menu_bar(app: &mut TrametesApp, ctx: &Context, frame: &mut Frame) {
    TopBottomPanel::top("top_menu_bar_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            // File
            ui.menu_button("File", |ui| {
                if ui.button("New...").clicked() {
                    commands::new(app);
                }

                if ui.button("Open...").clicked() {
                    commands::open(app);
                }

                ui.menu_button("Open Recent", |ui| {
                    // TODO
                    if ui.button("TODO put stuff here lol").clicked() {
                        commands::todo("open recent");
                    }
                });

                ui.separator();

                if ui.button("Save").clicked() {
                    commands::save(app);
                }

                if ui.button("Save As...").clicked() {
                    commands::save_as(app);
                }

                ui.separator();

                if ui.button("Settings...").clicked() {
                    commands::todo("settings");
                }

                // No "Quit" on the web
                if !frame.is_web() && ui.button("Quit").clicked() {
                    frame.close();
                }
            });

            // Edit
            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {
                    commands::todo("undo");
                }

                if ui.button("Redo").clicked() {
                    commands::todo("redo");
                }

                ui.separator();

                if ui.button("Cut").clicked() {
                    commands::todo("cut");
                }

                if ui.button("Copy").clicked() {
                    commands::copy(app);
                }

                if ui.button("Paste").clicked() {
                    commands::todo("paste");
                }

                if ui.button("Paste into New Image").clicked() {
                    commands::paste_into_new_image(app);
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
