use std::convert::TryInto;
use wasm_bindgen::Clamped;

pub type Pixel = [u8; 4];
#[derive(Debug, Clone)]
pub struct Pixels {
    pub v: Vec<Pixel>,
    pub w: u32,
    pub h: u32,
}

impl Pixels {
    pub fn grid_to_index(&self, x: u32, y: u32) -> usize {
        (y * self.w + x) as usize
    }
}

impl From<web_sys::ImageData> for Pixels {
    fn from(img: web_sys::ImageData) -> Self {
        let w = img.width();
        let h = img.height();
        let v = img
            .data()
            .chunks_exact(4)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<Pixel>>();
        Pixels { v, w, h }
    }
}

impl From<Pixels> for web_sys::ImageData {
    fn from(pixels: Pixels) -> Self {
        let w = pixels.w;
        let h = pixels.h;
        web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(
                pixels
                    .v
                    .iter()
                    .flat_map(|pixel| pixel.iter())
                    .cloned()
                    .collect::<Vec<u8>>()
                    .as_mut_slice(),
            ),
            w,
            h,
        )
        .unwrap()
    }
}
