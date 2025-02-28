use crate::cli::StreamScreenOptions;
use crate::ledwand_dither::*;
use image::{
    imageops::{resize, FilterType},
    DynamicImage, ImageBuffer, Luma, Rgb, Rgba,
};
use log::{error, info, warn};
use scap::{
    capturer::{Capturer, Options},
    frame::convert_bgra_to_rgb,
    frame::Frame,
};
use servicepoint::{Bitmap, Command, CompressionCode, Connection, Origin, FRAME_PACING, PIXEL_HEIGHT, PIXEL_WIDTH};
use std::time::Duration;

pub fn stream_window(connection: &Connection, options: StreamScreenOptions) {
    info!("Starting capture with options: {:?}", options);

    let capturer = match start_capture(&options) {
        Some(value) => value,
        None => return,
    };

    info!("now starting to stream images");
    loop {
        let mut frame = get_next_frame(&capturer);

        if !options.no_hist {
            histogram_correction(&mut frame);
        }

        let mut orig = frame.clone();

        if !options.no_blur {
            blur(&orig, &mut frame);
            std::mem::swap(&mut frame, &mut orig);
        }

        if !options.no_sharp {
            sharpen(&orig, &mut frame);
            std::mem::swap(&mut frame, &mut orig);
        }

        let bitmap = if options.no_dither {
            let cutoff = median_brightness(&orig);
            let bits = orig.iter().map(move |x| x > &cutoff).collect();
            Bitmap::from_bitvec(orig.width() as usize, bits)
        } else {
            ostromoukhov_dither(orig, u8::MAX / 2)
        };

        connection
            .send(Command::BitmapLinearWin(
                Origin::ZERO,
                bitmap.clone(),
                CompressionCode::Uncompressed,
            ))
            .expect("failed to send frame to display");
    }
}

/// returns next frame from the capturer, resized and grayscale
fn get_next_frame(capturer: &Capturer) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let frame = capturer.get_next_frame().expect("failed to capture frame");
    let frame = frame_to_image(frame);
    let frame = frame.grayscale().to_luma8();

    resize(
        &frame,
        PIXEL_WIDTH as u32,
        PIXEL_HEIGHT as u32,
        FilterType::Nearest,
    )
}

fn start_capture(options: &StreamScreenOptions) -> Option<Capturer> {
    if !scap::is_supported() {
        error!("platform not supported by scap");
        return None;
    }

    if !scap::has_permission() {
        warn!("requesting screen recording permission");
        if !scap::request_permission() {
            error!("screen recording ermission denied");
            return None;
        }
    }

    let mut capturer = Capturer::build(Options {
        fps: FRAME_PACING.div_duration_f32(Duration::from_secs(1)) as u32,
        show_cursor: options.pointer,
        output_type: scap::frame::FrameType::BGR0, // this is more like a suggestion
        ..Default::default()
    })
    .expect("failed to create screen capture");
    capturer.start_capture();
    Some(capturer)
}

fn frame_to_image(frame: Frame) -> DynamicImage {
    match frame {
        Frame::BGRx(frame) => bgrx_to_rgb(frame.width, frame.height, frame.data),
        Frame::RGBx(frame) => DynamicImage::from(
            ImageBuffer::<Rgba<_>, _>::from_raw(
                frame.width as u32,
                frame.height as u32,
                frame.data,
            )
            .unwrap(),
        ),
        Frame::BGR0(frame) => bgrx_to_rgb(frame.width, frame.height, frame.data),
        Frame::RGB(frame) => DynamicImage::from(
            ImageBuffer::<Rgb<_>, _>::from_raw(frame.width as u32, frame.height as u32, frame.data)
                .unwrap(),
        ),
        Frame::BGRA(frame) => bgrx_to_rgb(frame.width, frame.height, frame.data),
        Frame::YUVFrame(_) | Frame::XBGR(_) => panic!("unsupported frame format"),
    }
}

fn bgrx_to_rgb(width: i32, height: i32, data: Vec<u8>) -> DynamicImage {
    DynamicImage::from(
        ImageBuffer::<Rgb<_>, _>::from_raw(width as u32, height as u32, convert_bgra_to_rgb(data))
            .unwrap(),
    )
}
