//! Based on https://github.com/WarkerAnhaltRanger/CCCB_Ledwand

use image::GrayImage;
use log::debug;
use servicepoint::{Bitmap, DisplayBitVec, PIXEL_HEIGHT};

type GrayHistogram = [usize; 256];

struct HistogramCorrection {
    pre_offset: f32,
    post_offset: f32,
    factor: f32,
}

pub fn histogram_correction(image: &mut GrayImage) {
    let histogram = make_histogram(image);
    let correction = determine_histogram_correction(image, histogram);
    apply_histogram_correction(image, correction)
}

fn make_histogram(image: &GrayImage) -> GrayHistogram {
    let mut histogram = [0; 256];
    for pixel in image.pixels() {
        histogram[pixel.0[0] as usize] += 1;
    }
    histogram
}

fn determine_histogram_correction(
    image: &GrayImage,
    histogram: GrayHistogram,
) -> HistogramCorrection {
    let adjustment_pixels = image.len() / PIXEL_HEIGHT;

    let mut num_pixels = 0;
    let mut brightness = 0;

    let mincut = loop {
        num_pixels += histogram[brightness as usize];
        brightness += 1;
        if num_pixels >= adjustment_pixels {
            break u8::min(brightness, 20);
        }
    };

    let minshift = loop {
        num_pixels += histogram[brightness as usize];
        brightness += 1;
        if num_pixels >= 2 * adjustment_pixels {
            break u8::min(brightness, 64);
        }
    };

    brightness = u8::MAX;
    num_pixels = 0;
    let maxshift = loop {
        num_pixels += histogram[brightness as usize];
        brightness -= 1;
        if num_pixels >= 2 * adjustment_pixels {
            break u8::max(brightness, 192);
        }
    };

    let pre_offset = -(mincut as f32 / 2.);
    let post_offset = -(minshift as f32);
    let factor = (255.0 - post_offset) / maxshift as f32;
    HistogramCorrection {
        pre_offset,
        post_offset,
        factor,
    }
}

fn apply_histogram_correction(image: &mut GrayImage, correction: HistogramCorrection) {
    for pixel in image.pixels_mut() {
        let pixel = &mut pixel.0[0];
        let value =
            (*pixel as f32 + correction.pre_offset) * correction.factor + correction.post_offset;
        *pixel = value.clamp(0f32, u8::MAX as f32) as u8;
    }
}

pub fn median_brightness(image: &GrayImage) -> u8 {
    let histogram = make_histogram(image);
    let midpoint = image.len() / 2;

    debug_assert_eq!(
        image.len(),
        histogram.iter().copied().map(usize::from).sum()
    );

    let mut num_pixels = 0;
    for brightness in u8::MIN..=u8::MAX {
        num_pixels += histogram[brightness as usize];
        if num_pixels >= midpoint {
            return brightness;
        }
    }

    unreachable!("Somehow less pixels where counted in the histogram than exist in the image")
}

pub fn blur(source: &GrayImage, destination: &mut GrayImage) {
    assert_eq!(source.len(), destination.len());

    copy_border(source, destination);
    blur_inner_pixels(source, destination);
}

pub fn sharpen(source: &GrayImage, destination: &mut GrayImage) {
    assert_eq!(source.len(), destination.len());

    copy_border(source, destination);
    sharpen_inner_pixels(source, destination);
}

fn copy_border(source: &GrayImage, destination: &mut GrayImage) {
    let last_row = source.height() - 1;
    for x in 0..source.width() {
        destination[(x, 0)] = source[(x, 0)];
        destination[(x, last_row)] = source[(x, last_row)];
    }
    let last_col = source.width() - 1;
    for y in 0..source.height() {
        destination[(0, y)] = source[(0, y)];
        destination[(last_col, y)] = source[(last_col, y)];
    }
}

fn blur_inner_pixels(source: &GrayImage, destination: &mut GrayImage) {
    for y in 1..source.height() - 2 {
        for x in 1..source.width() - 2 {
            let weighted_sum = source.get_pixel(x - 1, y - 1).0[0] as u32
                + source.get_pixel(x, y - 1).0[0] as u32
                + source.get_pixel(x + 1, y - 1).0[0] as u32
                + source.get_pixel(x - 1, y).0[0] as u32
                + 8 * source.get_pixel(x, y).0[0] as u32
                + source.get_pixel(x + 1, y).0[0] as u32
                + source.get_pixel(x - 1, y + 1).0[0] as u32
                + source.get_pixel(x, y + 1).0[0] as u32
                + source.get_pixel(x + 1, y + 1).0[0] as u32;
            let blurred = weighted_sum / 16;
            destination.get_pixel_mut(x, y).0[0] =
                blurred.clamp(u8::MIN as u32, u8::MAX as u32) as u8;
        }
    }
}

fn sharpen_inner_pixels(source: &GrayImage, destination: &mut GrayImage) {
    for y in 1..source.height() - 2 {
        for x in 1..source.width() - 2 {
            let weighted_sum = -(source.get_pixel(x - 1, y - 1).0[0] as i32)
                - source.get_pixel(x, y - 1).0[0] as i32
                - source.get_pixel(x + 1, y - 1).0[0] as i32
                - source.get_pixel(x - 1, y).0[0] as i32
                + 9 * source.get_pixel(x, y).0[0] as i32
                - source.get_pixel(x + 1, y).0[0] as i32
                - source.get_pixel(x - 1, y + 1).0[0] as i32
                - source.get_pixel(x, y + 1).0[0] as i32
                - source.get_pixel(x + 1, y + 1).0[0] as i32;
            destination.get_pixel_mut(x, y).0[0] =
                weighted_sum.clamp(u8::MIN as i32, u8::MAX as i32) as u8;
        }
    }
}

pub(crate) fn ostromoukhov_dither(source: GrayImage, bias: u8) -> Bitmap {
    let width = source.width();
    let height = source.height();
    assert_eq!(width % 8, 0);

    let mut source = source.into_raw();
    let mut destination = DisplayBitVec::repeat(false, source.len());

    for y in 0..height as usize {
        let start = y * width as usize;
        let last_row = y == (height - 1) as usize;
        if y % 2 == 0 {
            for x in start..start + width as usize {
                ostromoukhov_dither_pixel(
                    &mut source,
                    &mut destination,
                    x,
                    width as usize,
                    last_row,
                    1,
                    bias,
                );
            }
        } else {
            for x in (start..start + width as usize).rev() {
                ostromoukhov_dither_pixel(
                    &mut source,
                    &mut destination,
                    x,
                    width as usize,
                    last_row,
                    -1,
                    bias,
                );
            }
        }
    }

    Bitmap::from_bitvec(width as usize, destination).unwrap()
}

#[inline]
fn ostromoukhov_dither_pixel(
    source: &mut [u8],
    destination: &mut DisplayBitVec,
    position: usize,
    width: usize,
    last_row: bool,
    direction: isize,
    bias: u8,
) {
    let (destination_value, error) = gray_to_bit(source[position], bias);
    destination.set(position, destination_value);

    let mut diffuse = |to: usize, mat: i16| {
        match source.get(to) {
            None => {
                // last row has a out of bounds error on the last pixel
                // TODO fix the iter bounds instead of ignoring here
            }
            Some(val) => {
                let diffuse_value = *val as i16 + mat;
                source[to] = diffuse_value.clamp(u8::MIN.into(), u8::MAX.into()) as u8;
            }
        };
    };

    let lookup = if destination_value {
        ERROR_DIFFUSION_MATRIX[error as usize].map(move |i| -i)
    } else {
        ERROR_DIFFUSION_MATRIX[error as usize]
    };
    diffuse((position as isize + direction) as usize, lookup[0]);

    if !last_row {
        debug!("begin");
        diffuse(
            ((position + width) as isize - direction) as usize,
            lookup[1],
        );
        debug!("mit");
        diffuse(((position + width) as isize) as usize, lookup[2]);
        debug!("end");
    }
}

fn gray_to_bit(old_pixel: u8, bias: u8) -> (bool, u8) {
    let destination_value = old_pixel > bias;
    let error = if destination_value {
        255 - old_pixel
    } else {
        old_pixel
    };
    (destination_value, error)
}

const ERROR_DIFFUSION_MATRIX: [[i16; 3]; 256] = [
    [0, 1, 0],
    [1, 0, 0],
    [1, 0, 1],
    [2, 0, 1],
    [2, 0, 2],
    [3, 0, 2],
    [4, 0, 2],
    [4, 1, 2],
    [5, 1, 2],
    [5, 2, 2],
    [5, 3, 2],
    [6, 3, 2],
    [6, 3, 3],
    [7, 3, 3],
    [7, 4, 3],
    [8, 4, 3],
    [8, 5, 3],
    [9, 5, 3],
    [9, 5, 4],
    [10, 6, 3],
    [10, 6, 4],
    [11, 7, 3],
    [11, 7, 4],
    [11, 8, 4],
    [12, 7, 5],
    [12, 7, 6],
    [12, 7, 7],
    [12, 7, 8],
    [12, 7, 9],
    [13, 7, 9],
    [13, 7, 10],
    [13, 7, 11],
    [13, 7, 12],
    [14, 7, 12],
    [14, 8, 12],
    [15, 8, 12],
    [15, 9, 12],
    [16, 9, 12],
    [16, 10, 12],
    [17, 10, 12],
    [17, 11, 12],
    [18, 12, 11],
    [19, 12, 11],
    [19, 13, 11],
    [20, 13, 11],
    [20, 14, 11],
    [21, 15, 10],
    [22, 15, 10],
    [22, 17, 9],
    [23, 17, 9],
    [24, 18, 8],
    [24, 19, 8],
    [25, 19, 8],
    [26, 20, 7],
    [26, 21, 7],
    [27, 22, 6],
    [28, 23, 5],
    [28, 24, 5],
    [29, 25, 4],
    [30, 26, 3],
    [31, 26, 3],
    [31, 28, 2],
    [32, 28, 2],
    [33, 29, 1],
    [34, 30, 0],
    [33, 31, 1],
    [32, 33, 1],
    [32, 33, 2],
    [31, 34, 3],
    [30, 36, 3],
    [29, 37, 4],
    [29, 37, 5],
    [28, 39, 5],
    [32, 34, 7],
    [37, 29, 8],
    [42, 23, 10],
    [46, 19, 11],
    [51, 13, 12],
    [52, 14, 13],
    [53, 13, 12],
    [53, 14, 13],
    [54, 14, 13],
    [55, 14, 13],
    [55, 14, 13],
    [56, 15, 14],
    [57, 14, 13],
    [56, 15, 15],
    [55, 17, 15],
    [54, 18, 16],
    [53, 20, 16],
    [52, 21, 17],
    [52, 22, 17],
    [51, 24, 17],
    [50, 25, 18],
    [49, 27, 18],
    [47, 29, 19],
    [48, 29, 19],
    [48, 29, 20],
    [49, 29, 20],
    [49, 30, 20],
    [50, 31, 20],
    [50, 31, 20],
    [51, 31, 20],
    [51, 31, 21],
    [52, 31, 21],
    [52, 32, 21],
    [53, 32, 21],
    [53, 32, 22],
    [55, 32, 21],
    [56, 31, 22],
    [58, 31, 21],
    [59, 30, 22],
    [61, 30, 21],
    [62, 29, 22],
    [64, 29, 21],
    [65, 28, 22],
    [67, 28, 21],
    [68, 27, 22],
    [70, 27, 21],
    [71, 26, 22],
    [73, 26, 21],
    [75, 25, 21],
    [76, 25, 21],
    [78, 24, 21],
    [80, 23, 21],
    [81, 23, 21],
    [83, 22, 21],
    [85, 21, 20],
    [85, 22, 21],
    [85, 22, 22],
    [84, 24, 22],
    [84, 24, 23],
    [84, 25, 23],
    [83, 27, 23],
    [83, 28, 23],
    [82, 29, 24],
    [82, 30, 24],
    [81, 31, 25],
    [80, 32, 26],
    [80, 33, 26],
    [79, 35, 26],
    [79, 36, 26],
    [78, 37, 27],
    [77, 38, 28],
    [77, 39, 28],
    [76, 41, 28],
    [75, 42, 29],
    [75, 43, 29],
    [74, 44, 30],
    [74, 45, 30],
    [75, 46, 30],
    [75, 46, 30],
    [76, 46, 30],
    [76, 46, 31],
    [77, 46, 31],
    [77, 47, 31],
    [78, 47, 31],
    [78, 47, 32],
    [79, 47, 32],
    [79, 48, 32],
    [80, 49, 32],
    [83, 46, 32],
    [86, 44, 32],
    [90, 42, 31],
    [93, 40, 31],
    [96, 39, 30],
    [100, 36, 30],
    [103, 35, 29],
    [106, 33, 29],
    [110, 30, 29],
    [113, 29, 28],
    [114, 29, 28],
    [115, 29, 28],
    [115, 29, 28],
    [116, 30, 29],
    [117, 29, 28],
    [117, 30, 29],
    [118, 30, 29],
    [119, 30, 29],
    [109, 43, 27],
    [100, 57, 23],
    [90, 71, 20],
    [80, 85, 17],
    [70, 99, 14],
    [74, 98, 12],
    [78, 97, 10],
    [81, 96, 9],
    [85, 95, 7],
    [89, 94, 5],
    [92, 93, 4],
    [96, 92, 2],
    [100, 91, 0],
    [100, 90, 2],
    [100, 88, 5],
    [100, 87, 7],
    [99, 86, 10],
    [99, 85, 12],
    [99, 84, 14],
    [99, 82, 17],
    [98, 81, 20],
    [98, 80, 22],
    [98, 79, 24],
    [98, 77, 27],
    [98, 76, 29],
    [97, 75, 32],
    [97, 73, 35],
    [97, 72, 37],
    [96, 71, 40],
    [96, 69, 43],
    [96, 67, 46],
    [96, 66, 48],
    [95, 65, 51],
    [95, 63, 54],
    [95, 61, 57],
    [94, 60, 60],
    [94, 58, 63],
    [94, 57, 65],
    [93, 55, 69],
    [93, 54, 71],
    [93, 52, 74],
    [92, 51, 77],
    [92, 49, 80],
    [91, 47, 84],
    [91, 46, 86],
    [93, 49, 82],
    [96, 52, 77],
    [98, 55, 73],
    [101, 58, 68],
    [104, 61, 63],
    [106, 65, 58],
    [109, 68, 53],
    [111, 71, 49],
    [114, 74, 44],
    [116, 78, 39],
    [118, 76, 40],
    [119, 74, 42],
    [120, 73, 43],
    [122, 71, 44],
    [123, 69, 46],
    [124, 67, 48],
    [125, 66, 49],
    [127, 64, 50],
    [128, 62, 52],
    [129, 60, 54],
    [131, 58, 55],
    [132, 57, 56],
    [136, 47, 63],
    [139, 38, 70],
    [143, 29, 76],
    [147, 19, 83],
    [151, 9, 90],
    [154, 0, 97],
    [160, 0, 92],
    [171, 0, 82],
    [183, 0, 71],
    [184, 0, 71],
];
