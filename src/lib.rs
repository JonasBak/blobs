mod filter;
mod pixel;

use filter::*;
use pixel::*;

use std::f64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn apply_filter(ctx: &web_sys::CanvasRenderingContext2d) {
    let (w, h) = {
        let canvas = ctx.canvas().unwrap();
        (canvas.width(), canvas.height())
    };
    let mut pixels: Pixels = ctx
        .get_image_data(0.0, 0.0, w as f64, h as f64)
        .unwrap()
        .into();

    Kernel3x3Filter {
        k: LAPLACIAN2,
        m: red_pixel_mapping,
        r: red_apply_to_pixel,
    }
    .apply(&mut pixels);

    Kernel3x3Filter {
        k: LAPLACIAN2,
        m: green_pixel_mapping,
        r: green_apply_to_pixel,
    }
    .apply(&mut pixels);

    Kernel3x3Filter {
        k: LAPLACIAN2,
        m: blue_pixel_mapping,
        r: blue_apply_to_pixel,
    }
    .apply(&mut pixels);

    ctx.put_image_data(&pixels.into(), 0.0, 0.0).unwrap();
}

#[wasm_bindgen(start)]
pub fn start() {
    console_log!("wasm working!");
}
