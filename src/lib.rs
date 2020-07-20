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

    LAPLACIAN2.apply(
        &mut pixels,
        |pixel: Pixel| (pixel[0] as i64 + pixel[1] as i64 + pixel[2] as i64) / 3,
        |pixel: &mut Pixel, v: i64| {
            let v = v.max(0).min(255) as u8;
            pixel[0] = v;
            pixel[1] = v;
            pixel[2] = v;
        },
    );

    ctx.put_image_data(&pixels.into(), 0.0, 0.0).unwrap();
}

#[wasm_bindgen(start)]
pub fn start() {
    console_log!("wasm working!");
}
