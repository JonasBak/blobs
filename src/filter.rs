use crate::*;

pub type Kernel3x3 = [[i8; 3]; 3];
pub type PixelMapping = fn(Pixel) -> i64;
pub type ApplyToPixel = fn(&mut Pixel, i64);

pub struct Kernel3x3Filter {
    pub k: Kernel3x3,
    pub m: PixelMapping,
    pub r: ApplyToPixel,
}

pub trait Filter {
    fn apply(&self, pixels: &mut Pixels);
}

impl Filter for Kernel3x3Filter {
    fn apply(&self, pixels: &mut Pixels) {
        let mapped = pixels
            .v
            .clone()
            .into_iter()
            .map(self.m)
            .collect::<Vec<i64>>();
        for y in 1..pixels.h - 1 {
            for x in 1..pixels.w - 1 {
                let mut acc = 0;
                for j in -1..=1_i32 {
                    for i in -1..=1_i32 {
                        acc += (self.k[(j + 1) as usize][(i + 1) as usize] as i64)
                            * mapped[pixels
                                .grid_to_index((x as i32 + i) as u32, (y as i32 + j) as u32)];
                    }
                }
                let i = pixels.grid_to_index(x, y);
                (self.r)(&mut pixels.v[i], acc);
            }
        }
    }
}

pub const LAPLACIAN1: Kernel3x3 = [[0, -1, 0], [-1, 4, -1], [0, -1, 0]];
pub const LAPLACIAN2: Kernel3x3 = [[-1, -1, -1], [-1, 8, -1], [-1, -1, -1]];

pub fn mean_pixel_mapping(pixel: Pixel) -> i64 {
    (pixel[0] as i64 + pixel[1] as i64 + pixel[2] as i64) / 3
}

pub fn red_pixel_mapping(pixel: Pixel) -> i64 {
    pixel[0] as i64
}

pub fn green_pixel_mapping(pixel: Pixel) -> i64 {
    pixel[1] as i64
}

pub fn blue_pixel_mapping(pixel: Pixel) -> i64 {
    pixel[2] as i64
}

pub fn equal_apply_to_pixel(pixel: &mut Pixel, v: i64) {
    let v = v.max(0).min(255) as u8;
    pixel[0] = v;
    pixel[1] = v;
    pixel[2] = v;
}

pub fn red_apply_to_pixel(pixel: &mut Pixel, v: i64) {
    let v = v.max(0).min(255) as u8;
    pixel[0] = v;
}

pub fn green_apply_to_pixel(pixel: &mut Pixel, v: i64) {
    let v = v.max(0).min(255) as u8;
    pixel[1] = v;
}

pub fn blue_apply_to_pixel(pixel: &mut Pixel, v: i64) {
    let v = v.max(0).min(255) as u8;
    pixel[2] = v;
}
