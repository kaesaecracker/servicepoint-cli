use crate::{
    cli::ImageProcessingOptions,
    ledwand_dither::{blur, histogram_correction, median_brightness, ostromoukhov_dither, sharpen},
};
use fast_image_resize::{ResizeOptions, Resizer};
use image::{DynamicImage, GrayImage};
use log::{debug, trace};
use servicepoint::{Bitmap, Grid, PIXEL_HEIGHT, PIXEL_WIDTH, TILE_HEIGHT, TILE_SIZE};
use std::{default::Default, time::Instant};

#[derive(Debug)]
pub struct ImageProcessingPipeline {
    options: ImageProcessingOptions,
    resizer: Resizer,
    render_size: (usize, usize),
}

const SPACER_HEIGHT: usize = TILE_SIZE / 2;

impl ImageProcessingPipeline {
    pub fn new(options: ImageProcessingOptions) -> Self {
        debug!("Creating image pipeline: {:?}", options);

        let spacers_height = if options.no_spacers {
            0
        } else {
            SPACER_HEIGHT * (TILE_HEIGHT - 1)
        };

        Self {
            options,
            resizer: Resizer::new(),
            render_size: (PIXEL_WIDTH, PIXEL_HEIGHT + spacers_height),
        }
    }

    pub fn process(&mut self, frame: DynamicImage) -> Bitmap {
        let start_time = Instant::now();

        let frame = self.resize_grayscale(frame);
        let frame = self.grayscale_processing(frame);
        let mut result = self.grayscale_to_bitmap(frame);

        if !self.options.no_spacers {
            result = Self::remove_spacers(result);
        }

        trace!("pipeline took {:?}", start_time.elapsed());
        result
    }

    fn resize_grayscale(&mut self, frame: DynamicImage) -> GrayImage {
        let start_time = Instant::now();

        let (scaled_width, scaled_height) = self.fit_size((frame.width(), frame.height()));
        let mut dst_image = DynamicImage::new(scaled_width, scaled_height, frame.color());

        self.resizer
            .resize(&frame, &mut dst_image, &ResizeOptions::default())
            .expect("image resize failed");

        trace!("resizing took {:?}", start_time.elapsed());

        let start_time = Instant::now();
        let result = dst_image.into_luma8();
        trace!("grayscale took {:?}", start_time.elapsed());

        result
    }

    fn grayscale_processing(&self, mut frame: GrayImage) -> GrayImage {
        let start_time = Instant::now();
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

        trace!("image processing took {:?}", start_time.elapsed());
        orig
    }

    fn grayscale_to_bitmap(&self, orig: GrayImage) -> Bitmap {
        let start_time = Instant::now();
        let result = if self.options.no_dither {
            let cutoff = median_brightness(&orig);
            let bits = orig.iter().map(move |x| x > &cutoff).collect();
            Bitmap::from_bitvec(orig.width() as usize, bits)
        } else {
            ostromoukhov_dither(orig, u8::MAX / 2)
        };
        trace!("bitmap conversion took {:?}", start_time.elapsed());
        result
    }

    fn remove_spacers(source: Bitmap) -> Bitmap {
        let start_time = Instant::now();

        let full_tile_rows_with_spacers = source.height() / (TILE_SIZE + SPACER_HEIGHT);
        let remaining_pixel_rows = source.height() % (TILE_SIZE + SPACER_HEIGHT);
        let total_spacer_height = full_tile_rows_with_spacers * SPACER_HEIGHT
            + remaining_pixel_rows.saturating_sub(TILE_SIZE);
        let height_without_spacers = source.height() - total_spacer_height;
        trace!(
            "spacers take up {total_spacer_height}, resulting in height {height_without_spacers}"
        );

        let mut result = Bitmap::new(source.width(), height_without_spacers);

        let mut source_y = 0;
        for result_y in 0..result.height() {
            for x in 0..result.width() {
                result.set(x, result_y, source.get(x, source_y));
            }

            if result_y != 0 && result_y % TILE_SIZE == 0 {
                source_y += SPACER_HEIGHT;
            }
            source_y += 1;
        }

        trace!("removing spacers took {:?}", start_time.elapsed());
        result
    }

    fn fit_size(&self, source: (u32, u32)) -> (u32, u32) {
        let (source_width, source_height) = source;
        let (target_width, target_height) = self.render_size;
        debug_assert_eq!(target_width % TILE_SIZE, 0);

        let width_scale = target_width as f32 / source_width as f32;
        let height_scale = target_height as f32 / source_height as f32;
        let scale = f32::min(width_scale, height_scale);

        let height = (source_height as f32 * scale) as u32;
        let mut width = (source_width as f32 * scale) as u32;

        if width % TILE_SIZE as u32 != 0 {
            // because we do not have many pixels, round up even if it is a worse fit
            width += 8 - width % 8;
        }

        let result = (width, height);
        trace!(
            "scaling {:?} to {:?} to fit {:?}",
            source,
            result,
            self.render_size
        );
        result
    }
}
