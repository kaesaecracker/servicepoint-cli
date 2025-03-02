use crate::{
    cli::{ImageProcessingOptions, StreamScreenOptions},
    image_processing::ImageProcessingPipeline,
};
use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use log::{debug, error, info, trace, warn};
use scap::{
    capturer::{Capturer, Options},
    frame::convert_bgra_to_rgb,
    frame::Frame,
};
use servicepoint::{Command, CompressionCode, Connection, Origin, FRAME_PACING};
use std::time::{Duration, Instant};

pub fn stream_window(
    connection: &Connection,
    options: StreamScreenOptions,
    processing_options: ImageProcessingOptions,
) {
    info!("Starting capture with options: {:?}", options);
    let capturer = match start_capture(&options) {
        Some(value) => value,
        None => return,
    };

    let mut pipeline = ImageProcessingPipeline::new(processing_options);

    info!("now starting to stream images");
    loop {
        let start = Instant::now();

        let frame = capture_frame(&capturer);
        let frame = frame_to_image(frame);
        let bitmap = pipeline.process(frame);

        trace!("bitmap ready to send in: {:?}", start.elapsed());

        connection
            .send(Command::BitmapLinearWin(
                Origin::ZERO,
                bitmap.clone(),
                CompressionCode::default(),
            ))
            .expect("failed to send frame to display");

        debug!("frame time: {:?}", start.elapsed());
    }
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

    // all options are more like a suggestion
    let mut capturer = Capturer::build(Options {
        fps: FRAME_PACING.div_duration_f32(Duration::from_secs(1)) as u32,
        show_cursor: options.pointer,
        output_type: scap::frame::FrameType::BGR0,
        ..Default::default()
    })
    .expect("failed to create screen capture");
    capturer.start_capture();
    Some(capturer)
}

fn capture_frame(capturer: &Capturer) -> Frame {
    let start_time = Instant::now();
    let result = capturer.get_next_frame().expect("failed to capture frame");
    trace!("capture took: {:?}", start_time.elapsed());
    result
}

fn frame_to_image(frame: Frame) -> DynamicImage {
    let start_time = Instant::now();
    let result = match frame {
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
    };
    trace!("conversion to image took: {:?}", start_time.elapsed());
    result
}

fn bgrx_to_rgb(width: i32, height: i32, data: Vec<u8>) -> DynamicImage {
    DynamicImage::from(
        ImageBuffer::<Rgb<_>, _>::from_raw(width as u32, height as u32, convert_bgra_to_rgb(data))
            .unwrap(),
    )
}
