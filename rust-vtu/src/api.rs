#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn import_vtu_bytes(data: Vec<u8>) {
    info!("Received {:?} VTU bytes", data.len());
}
