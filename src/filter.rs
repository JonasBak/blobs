use crate::*;

pub type Kernel3x3 = [[i8; 3]; 3];
pub type PixelMapping = fn(Pixel) -> i64;
pub type ApplyToPixel = fn(&mut Pixel, i64);

pub const LAPLACIAN1: Kernel3x3 = [[0, -1, 0], [-1, 4, -1], [0, -1, 0]];
pub const LAPLACIAN2: Kernel3x3 = [[-1, -1, -1], [-1, 8, -1], [-1, -1, -1]];

pub trait Filter {
    fn apply(&self, pixels: &mut Pixels, mapping: PixelMapping, reverse: ApplyToPixel);
}

impl Filter for Kernel3x3 {
    fn apply(&self, pixels: &mut Pixels, mapping: PixelMapping, reverse: ApplyToPixel) {
        let mapped = pixels
            .v
            .clone()
            .into_iter()
            .map(mapping)
            .collect::<Vec<i64>>();
        for y in 1..pixels.h - 1 {
            for x in 1..pixels.w - 1 {
                let mut acc = 0;
                for i in -1..=1_i32 {
                    for j in -1..=1_i32 {
                        acc += (self[(j + 1) as usize][(i + 1) as usize] as i64)
                            * mapped[pixels
                                .grid_to_index((x as i32 + i) as u32, (y as i32 + j) as u32)];
                    }
                }
                let i = pixels.grid_to_index(x, y);
                reverse(&mut pixels.v[i], acc);
            }
        }
    }
}
