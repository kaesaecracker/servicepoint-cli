use crate::{
    cli::ImageProcessingOptions,
    ledwand_dither::{blur, histogram_correction, median_brightness, ostromoukhov_dither, sharpen},
};
use image::{
    imageops::{resize, FilterType},
    DynamicImage, ImageBuffer, Luma,
};
use log::{debug, trace};
use servicepoint::{Bitmap, Grid, PIXEL_HEIGHT, PIXEL_WIDTH, TILE_HEIGHT, TILE_SIZE};
use std::time::Instant;

#[derive(Debug)]
pub struct ImageProcessingPipeline {
    options: ImageProcessingOptions,
}

const SPACER_HEIGHT: usize = TILE_SIZE / 2;
const PIXEL_HEIGHT_INCLUDING_SPACERS: usize = SPACER_HEIGHT * (TILE_HEIGHT - 1) + PIXEL_HEIGHT;

impl ImageProcessingPipeline {
    pub fn new(options: ImageProcessingOptions) -> Self {
        debug!("Creating image pipeline: {:?}", options);
        Self { options }
    }

    pub fn process(&self, frame: DynamicImage) -> Bitmap {
        let start_time = Instant::now();

        let frame = self.resize_grayscale(frame);
        let frame = self.grayscale_processing(frame);
        let mut result = self.grayscale_to_bitmap(frame);

        if !self.options.no_spacers {
            result = Self::remove_spacers(result);
        }

        trace!("image processing took {:?}", start_time.elapsed());
        result
    }

    fn resize_grayscale(&self, frame: DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        // TODO: keep aspect ratio
        // TODO: make it work for non-maximum sizes

        let frame = frame.grayscale().to_luma8();

        let target_height = if self.options.no_spacers {
            PIXEL_HEIGHT
        } else {
            PIXEL_HEIGHT_INCLUDING_SPACERS
        };

        resize(
            &frame,
            PIXEL_WIDTH as u32,
            target_height as u32,
            FilterType::Nearest,
        )
    }

    fn grayscale_processing(
        &self,
        mut frame: ImageBuffer<Luma<u8>, Vec<u8>>,
    ) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        if !self.options.no_hist {
            histogram_correction(&mut frame);
        }

        let mut orig = frame.clone();

        if !self.options.no_blur {
            blur(&orig, &mut frame);
            std::mem::swap(&mut frame, &mut orig);
        }

        if !self.options.no_sharp {
            sharpen(&orig, &mut frame);
            std::mem::swap(&mut frame, &mut orig);
        }
        orig
    }

    fn grayscale_to_bitmap(&self, orig: ImageBuffer<Luma<u8>, Vec<u8>>) -> Bitmap {
        if self.options.no_dither {
            let cutoff = median_brightness(&orig);
            let bits = orig.iter().map(move |x| x > &cutoff).collect();
            Bitmap::from_bitvec(orig.width() as usize, bits)
        } else {
            ostromoukhov_dither(orig, u8::MAX / 2)
        }
    }

    fn remove_spacers(bitmap: Bitmap) -> Bitmap {
        let mut result = Bitmap::max_sized();

        let mut source_y = 0;
        for result_y in 0..result.height() {
            if result_y != 0 && result_y % TILE_SIZE == 0 {
                source_y += 4;
            }

            for x in 0..result.width() {
                result.set(x, result_y, bitmap.get(x, source_y));
            }

            source_y += 1;
        }

        result
    }
}
