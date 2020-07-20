use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn test_canvas(ctx: &web_sys::CanvasRenderingContext2d) {
    ctx.begin_path();

    ctx.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    ctx.move_to(110.0, 75.0);
    ctx.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    ctx.move_to(65.0, 65.0);
    ctx.arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    ctx.move_to(95.0, 65.0);
    ctx.arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    ctx.stroke();
}

#[wasm_bindgen]
pub fn flip_canvas(ctx: &web_sys::CanvasRenderingContext2d) {
    let (w, h) = {
        let canvas = ctx.canvas().unwrap();
        (canvas.width(), canvas.height())
    };
    let image_data = ctx.get_image_data(0.0, 0.0, w as f64, h as f64).unwrap();
    let flipped_image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(
            image_data
                .data()
                .chunks(4)
                .rev()
                .flatten()
                .cloned()
                .collect::<Vec<u8>>()
                .as_mut_slice(),
        ),
        w,
        h,
    )
    .unwrap();
    ctx.put_image_data(&flipped_image_data, 0.0, 0.0).unwrap();
}

#[wasm_bindgen(start)]
pub fn start() {
    console_log!("wasm working!");
}
