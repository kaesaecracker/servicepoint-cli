//! Based on https://github.com/WarkerAnhaltRanger/CCCB_Ledwand

use image::{GenericImage, GrayImage};
use servicepoint::{PIXEL_HEIGHT, PIXEL_WIDTH};

pub struct LedwandDither {
    options: LedwandDitherOptions,
    tmpbuf: GrayImage,
}

#[derive(Debug, Default)]
pub struct LedwandDitherOptions {
    pub size: Option<(u32, u32)>,
}

type GrayHistogram = [usize; 256];

struct HistogramCorrection {
    pre_offset: f32,
    post_offset: f32,
    factor: f32,
}

impl LedwandDither {
    pub fn new(options: LedwandDitherOptions) -> Self {
        let (width, height) = options
            .size
            .unwrap_or((PIXEL_WIDTH as u32, PIXEL_HEIGHT as u32));
        Self {
            tmpbuf: GrayImage::new(width, height),
            options,
        }
    }

    pub fn histogram_correction(image: &mut GrayImage) {
        let histogram = Self::make_histogram(image);
        let correction = Self::determine_histogram_correction(image, histogram);
        Self::apply_histogram_correction(image, correction)
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
            let value = (*pixel as f32 + correction.pre_offset) * correction.factor
                + correction.post_offset;
            *pixel = value.clamp(0f32, u8::MAX as f32) as u8;
        }
    }

    pub fn median_brightness(image: &GrayImage) -> u8 {
        let histogram = Self::make_histogram(image);
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

        Self::copy_border(source, destination);
        Self::blur_inner_pixels(source, destination);
    }

    fn copy_border(source: &GrayImage, destination: &mut GrayImage) {
        let last_row = source.height() -1;
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
                destination.get_pixel_mut(x, y).0[0] = blurred.clamp(u8::MIN as u32, u8::MAX as u32) as u8;
            }
        }
    }
}
