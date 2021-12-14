use wasm_bindgen::prelude::*;
use anyhow::Result;
use water_levels_core::Terrain;

#[wasm_bindgen(getter_with_clone)]
pub struct Res {
    pub res: Option<Vec<f64>>,
    pub err: Option<String>,
}

fn calculate_(segments: &[f64], hours: f64) -> Result<Vec<f64>> {
    Terrain::new(segments)?.fill(hours)
}

#[wasm_bindgen]
pub fn calculate(segments: &[f64], level: f64) -> Res {
    console_error_panic_hook::set_once();
    match calculate_(segments, level) {
        Ok(r) => Res {
            res: Some(r),
            err: None,
        },
        Err(e) => Res {
            res: None,
            err: Some(format!("{:?}", e)),
        }
    }
}
