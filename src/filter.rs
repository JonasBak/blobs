use crate::*;

macro_rules! impl_convolution_kernel {
    ($type:ident, $offset:expr) => {
        impl Convolution for $type {
            fn apply_to_pixel(&self, values: &[f64], pixels: &Pixels, x: u32, y: u32) -> f64 {
                if x < $offset || y < $offset || x >= pixels.w - $offset || y >= pixels.h - $offset
                {
                    return values[pixels.grid_to_index(x, y)];
                }
                let mut acc = 0.0;
                for j in -$offset..=$offset {
                    for i in -$offset..=$offset {
                        acc += (self[(j + $offset) as usize][(i + $offset) as usize] as f64)
                            * values[pixels
                                .grid_to_index((x as i32 + i) as u32, (y as i32 + j) as u32)];
                    }
                }
                acc
            }
        }
    };
}

pub type PixelMapping = fn(Pixel) -> f64;
pub type ApplyToPixel = fn(&mut Pixel, f64);

pub type Kernel3x3 = [[f64; 3]; 3];
pub type Kernel5x5 = [[f64; 5]; 5];
pub type Kernel7x7 = [[f64; 7]; 7];
pub type Kernel9x9 = [[f64; 9]; 9];

impl_convolution_kernel!(Kernel3x3, 1);
impl_convolution_kernel!(Kernel5x5, 2);
impl_convolution_kernel!(Kernel7x7, 3);
impl_convolution_kernel!(Kernel9x9, 4);

pub trait Filter {
    fn apply(&self, pixels: &mut Pixels);
}

pub trait Convolution {
    fn apply_to_pixel(&self, values: &[f64], pixels: &Pixels, x: u32, y: u32) -> f64;
}

pub struct ConvolutionFilter {
    pub k: &'static dyn Convolution,
    pub m: PixelMapping,
    pub r: ApplyToPixel,
}

impl Filter for ConvolutionFilter {
    fn apply(&self, pixels: &mut Pixels) {
        let mapped = pixels
            .v
            .clone()
            .into_iter()
            .map(self.m)
            .collect::<Vec<f64>>();
        for y in 0..pixels.h {
            for x in 0..pixels.w {
                let i = pixels.grid_to_index(x, y);
                let v = self.k.apply_to_pixel(&mapped, &pixels, x, y);
                (self.r)(&mut pixels.v[i], v);
            }
        }
    }
}

pub const LAPLACIAN1: Kernel3x3 = [[0.0, -1.0, 0.0], [-1.0, 4.0, -1.0], [0.0, -1.0, 0.0]];
pub const LAPLACIAN2: Kernel3x3 = [[-1.0, -1.0, -1.0], [-1.0, 8.0, -1.0], [-1.0, -1.0, -1.0]];

pub const LOG_5x5: Kernel5x5 = [
    [0.0, 0.0, 1.0, 0.0, 0.0],
    [0.0, 1.0, 2.0, 1.0, 0.0],
    [1.0, 2.0, -16.0, 2.0, 1.0],
    [0.0, 1.0, 2.0, 1.0, 0.0],
    [0.0, 0.0, 1.0, 0.0, 0.0],
];
pub const GAUSSIAN_5x5: Kernel5x5 = [
    [
        1.0 / 273.0,
        4.0 / 273.0,
        7.0 / 273.0,
        4.0 / 273.0,
        1.0 / 273.0,
    ],
    [
        4.0 / 273.0,
        16.0 / 273.0,
        26.0 / 273.0,
        16.0 / 273.0,
        4.0 / 273.0,
    ],
    [
        7.0 / 273.0,
        26.0 / 273.0,
        41.0 / 273.0,
        26.0 / 273.0,
        7.0 / 273.0,
    ],
    [
        4.0 / 273.0,
        16.0 / 273.0,
        26.0 / 273.0,
        16.0 / 273.0,
        4.0 / 273.0,
    ],
    [
        1.0 / 273.0,
        4.0 / 273.0,
        7.0 / 273.0,
        4.0 / 273.0,
        1.0 / 273.0,
    ],
];

pub const LOG_9x9: Kernel9x9 = [
    [0.0, 1.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 0.0],
    [1.0, 2.0, 4.0, 5.0, 5.0, 5.0, 4.0, 2.0, 1.0],
    [1.0, 4.0, 5.0, 3.0, 0.0, 3.0, 5.0, 4.0, 1.0],
    [2.0, 5.0, 3.0, -12.0, -24.0, -12.0, 3.0, 5.0, 2.0],
    [2.0, 5.0, 0.0, -24.0, -40.0, -24.0, 0.0, 5.0, 2.0],
    [2.0, 5.0, 3.0, -12.0, -24.0, -12.0, 3.0, 5.0, 2.0],
    [1.0, 4.0, 5.0, 3.0, 0.0, 3.0, 5.0, 4.0, 1.0],
    [1.0, 2.0, 4.0, 5.0, 5.0, 5.0, 4.0, 2.0, 1.0],
    [0.0, 1.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 0.0],
];

pub fn mean_pixel_mapping(pixel: Pixel) -> f64 {
    (pixel[0] as f64 + pixel[1] as f64 + pixel[2] as f64) / 3.0
}

pub fn red_pixel_mapping(pixel: Pixel) -> f64 {
    pixel[0] as f64
}

pub fn green_pixel_mapping(pixel: Pixel) -> f64 {
    pixel[1] as f64
}

pub fn blue_pixel_mapping(pixel: Pixel) -> f64 {
    pixel[2] as f64
}

pub fn equal_apply_to_pixel(pixel: &mut Pixel, v: f64) {
    let v = v.max(0.0).min(255.0) as u8;
    pixel[0] = v;
    pixel[1] = v;
    pixel[2] = v;
}

pub fn red_apply_to_pixel(pixel: &mut Pixel, v: f64) {
    let v = v.max(0.0).min(255.0) as u8;
    pixel[0] = v;
}

pub fn green_apply_to_pixel(pixel: &mut Pixel, v: f64) {
    let v = v.max(0.0).min(255.0) as u8;
    pixel[1] = v;
}

pub fn blue_apply_to_pixel(pixel: &mut Pixel, v: f64) {
    let v = v.max(0.0).min(255.0) as u8;
    pixel[2] = v;
}
