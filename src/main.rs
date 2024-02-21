#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

//use anyhow::{Context, Result};
use anyhow::anyhow;
use image;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TileMapperError {
    #[error("Failed to load icon when launching application")]
    IconLoadFail(#[from] image::ImageError),
    #[error("Failed to launch application")]
    ApplicationLaunchFail(#[from] eframe::Error),
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), anyhow::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let icon = eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
        .map_err(TileMapperError::IconLoadFail)
        .map_err(|e| anyhow!(e.to_string()))?;

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "TileMapper",
        native_options,
        Box::new(|cc| Box::new(tilemapper::TileMapper::new(cc))),
    )
    .map_err(TileMapperError::ApplicationLaunchFail)
    .map_err(|e| anyhow!(e.to_string()))?;

    Ok(())
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), anyhow::Error> {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(tilemapper::TileMapper::new(cc))),
            )
            .await
            .map_err(TileMapperError::ApplicationLaunchFail)
            .map_err(|e| anyhow!(e.to_string()))
    });
}
