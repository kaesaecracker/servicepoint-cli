use crate::cli::ImageProcessingOptions;
use crate::ledwand_dither::{
    blur, histogram_correction, median_brightness, ostromoukhov_dither, sharpen,
};
use image::imageops::{resize, FilterType};
use image::{imageops, DynamicImage, ImageBuffer, Luma};
use servicepoint::{Bitmap, PIXEL_HEIGHT, PIXEL_WIDTH};

pub struct ImageProcessingPipeline {
    options: ImageProcessingOptions,
}

impl ImageProcessingPipeline {
    pub fn new(options: ImageProcessingOptions) -> Self {
        Self { options }
    }

    pub fn process(&self, frame: DynamicImage) -> Bitmap {
        let frame = Self::resize_grayscale(&frame);
        let frame = self.grayscale_processing(frame);
        self.grayscale_to_bitmap(frame)
    }

    fn resize_grayscale(frame: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let frame = imageops::grayscale(&frame);
        let frame = resize(
            &frame,
            PIXEL_WIDTH as u32,
            PIXEL_HEIGHT as u32,
            FilterType::Nearest,
        );
        frame
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

    fn grayscale_to_bitmap(&self, mut orig: ImageBuffer<Luma<u8>, Vec<u8>>) -> Bitmap {
        if self.options.no_dither {
            let cutoff = median_brightness(&orig);
            let bits = orig.iter().map(move |x| x > &cutoff).collect();
            Bitmap::from_bitvec(orig.width() as usize, bits)
        } else {
            ostromoukhov_dither(orig, u8::MAX / 2)
        }
    }
}
