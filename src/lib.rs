mod filter;
mod pixel;

use filter::*;
use pixel::*;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const EDGE_DETECTION_GRAYSCALE: ConvolutionFilter = ConvolutionFilter {
    k: &LOG_5x5,
    m: mean_pixel_mapping,
    r: equal_apply_to_pixel,
};
const GAUSSIAN_BLUR_GRAYSCALE: ConvolutionFilter = ConvolutionFilter {
    k: &GAUSSIAN_5x5,
    m: mean_pixel_mapping,
    r: equal_apply_to_pixel,
};
const EDGE_DETECTION_RED: ConvolutionFilter = ConvolutionFilter {
    k: &LOG_5x5,
    m: red_pixel_mapping,
    r: red_apply_to_pixel,
};
const EDGE_DETECTION_GREEN: ConvolutionFilter = ConvolutionFilter {
    k: &LOG_5x5,
    m: green_pixel_mapping,
    r: green_apply_to_pixel,
};
const EDGE_DETECTION_BLUE: ConvolutionFilter = ConvolutionFilter {
    k: &LOG_5x5,
    m: blue_pixel_mapping,
    r: blue_apply_to_pixel,
};
const EDGE_DETECTION_RGB: ChainFilter = ChainFilter(&[
    &EDGE_DETECTION_RED,
    &EDGE_DETECTION_GREEN,
    &EDGE_DETECTION_BLUE,
]);

const FILTERS: &[(&str, &dyn Filter)] = &[
    ("No filter", &NopFilter()),
    (
        "Edge detection, laplacean of gaussian (grayscale)",
        &EDGE_DETECTION_GRAYSCALE,
    ),
    (
        "Edge detection, laplacean of gaussian (rgb channels)",
        &EDGE_DETECTION_RGB,
    ),
    ("Gaussian blur (grayscale)", &GAUSSIAN_BLUR_GRAYSCALE),
];

#[wasm_bindgen]
pub fn get_filters() -> Vec<JsValue> {
    FILTERS
        .iter()
        .map(|(name, _)| JsValue::from_str(name))
        .collect()
}

#[wasm_bindgen]
pub fn apply_filter(ctx: &web_sys::CanvasRenderingContext2d, filter: usize) {
    let (w, h) = {
        let canvas = ctx.canvas().unwrap();
        (canvas.width(), canvas.height())
    };
    let mut pixels: Pixels = ctx
        .get_image_data(0.0, 0.0, w as f64, h as f64)
        .unwrap()
        .into();

    FILTERS[filter].1.apply(&mut pixels);

    ctx.put_image_data(&pixels.into(), 0.0, 0.0).unwrap();
}

#[wasm_bindgen(start)]
pub fn start() {
    console_log!("wasm working!");
}
