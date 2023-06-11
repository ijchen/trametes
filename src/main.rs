#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{NativeOptions, Theme};
use egui::Vec2;
use trametes::TrametesApp;

/// Compile to native (desktop)
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    eframe::run_native(
        "Trametes",
        NativeOptions {
            maximized: true, // TODO this doesn't work, temporarily using hackey workaround with infinite initial_window_size
            drag_and_drop_support: true,
            icon_data: None, // TODO add a nice icon
            initial_window_size: Some(Vec2::new(f32::INFINITY, f32::INFINITY)), // TODO this is a hackey workaround for the fact that `maximized` doesn't seem to be working
            min_window_size: None, // TODO We probably want a minimum size
            max_window_size: None,
            resizable: true,
            transparent: false,
            vsync: true,
            follow_system_theme: false, // TODO properly implement custom themes
            default_theme: Theme::Dark,
            centered: true,
            ..Default::default()
        },
        Box::new(|cc| Box::new(TrametesApp::new(cc))),
    )
}

/// Compile to the web using trunk
#[cfg(target_arch = "wasm32")]
fn main_web() -> eframe::Result<()> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(TrametesApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
