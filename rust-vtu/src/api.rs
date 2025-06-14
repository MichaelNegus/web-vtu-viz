#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn import_file_path(path: &str) {
    info!("Received file path: {}", path);
}
