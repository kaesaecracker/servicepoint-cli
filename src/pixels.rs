use crate::{
    cli::{ImageProcessingOptions, PixelCommand, SendImageOptions},
    image_processing::ImageProcessingPipeline,
    stream_window::stream_window,
    transport::Transport,
};
use ffmpeg_next as ffmpeg;
use image::{DynamicImage, RgbImage};
use log::info;
use servicepoint::{
    BinaryOperation, BitVecCommand, BitmapCommand, ClearCommand, CompressionCode, DisplayBitVec,
    Origin, PIXEL_COUNT,
};

pub(crate) fn pixels(connection: &Transport, pixel_command: PixelCommand) {
    match pixel_command {
        PixelCommand::Off => pixels_off(connection),
        PixelCommand::Flip => pixels_invert(connection),
        PixelCommand::On => pixels_on(connection),
        PixelCommand::Image {
            image_processing_options: processing_options,
            send_image_options: image_options,
        } => pixels_image(connection, image_options, processing_options),
        PixelCommand::Screen {
            stream_options,
            image_processing,
        } => stream_window(connection, stream_options, image_processing),
        PixelCommand::Video {
            image_processing_options: processing_options,
            send_image_options: image_options,
        } => pixels_video(connection, image_options, processing_options),
    }
}

fn pixels_on(connection: &Transport) {
    let mask = DisplayBitVec::repeat(true, PIXEL_COUNT);
    let command = BitVecCommand {
        offset: 0,
        bitvec: mask,
        compression: CompressionCode::Lzma,
        operation: BinaryOperation::Overwrite,
    };
    connection
        .send_command(command)
        .expect("could not send command");
    info!("turned on all pixels")
}

fn pixels_invert(connection: &Transport) {
    let mask = DisplayBitVec::repeat(true, PIXEL_COUNT);
    let command = BitVecCommand {
        offset: 0,
        bitvec: mask,
        compression: CompressionCode::Lzma,
        operation: BinaryOperation::Xor,
    };
    connection
        .send_command(command)
        .expect("could not send command");
    info!("inverted all pixels");
}

pub(crate) fn pixels_off(connection: &Transport) {
    connection
        .send_command(ClearCommand)
        .expect("failed to clear pixels");
    info!("reset pixels");
}

fn pixels_image(
    connection: &Transport,
    options: SendImageOptions,
    processing_options: ImageProcessingOptions,
) {
    let image = image::open(&options.file_name).expect("failed to open image file");
    let mut pipeline = ImageProcessingPipeline::new(processing_options);
    let bitmap = pipeline.process(image);
    connection
        .send_command(BitmapCommand {
            origin: Origin::ZERO,
            bitmap,
            compression: CompressionCode::default(),
        })
        .expect("failed to send image command");
    info!("sent image to display");
}

fn pixels_video(
    connection: &Transport,
    options: SendImageOptions,
    processing_options: ImageProcessingOptions,
) {
    ffmpeg::init().unwrap();

    let mut ictx =
        ffmpeg::format::input(&options.file_name).expect("failed to open video input file");

    let input = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)
        .expect("could not get video stream from input file");
    let video_stream_index = input.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())
        .expect("could not extract video context from parameters");
    let mut decoder = context_decoder
        .decoder()
        .video()
        .expect("failed to create decoder for video stream");

    let src_width = decoder.width();
    let src_height = decoder.height();

    let mut scaler = ffmpeg::software::scaling::Context::get(
        decoder.format(),
        src_width,
        src_height,
        ffmpeg::format::Pixel::RGB24,
        src_width,
        src_height,
        ffmpeg::software::scaling::Flags::BILINEAR,
    )
    .expect("failed to create scaling context");

    let mut frame_index = 0;

    let mut processing_pipeline = ImageProcessingPipeline::new(processing_options);

    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
            let mut decoded = ffmpeg::util::frame::video::Video::empty();
            let mut rgb_frame = ffmpeg::util::frame::video::Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                scaler
                    .run(&decoded, &mut rgb_frame)
                    .expect("failed to scale frame");

                let image = RgbImage::from_raw(src_width, src_height, rgb_frame.data(0).to_owned())
                    .expect("could not read rgb data to image");
                let image = DynamicImage::from(image);
                let bitmap = processing_pipeline.process(image);
                connection
                    .send_command(BitmapCommand::from(bitmap))
                    .expect("failed to send image command");

                frame_index += 1;
            }
            Ok(())
        };

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder
                .send_packet(&packet)
                .expect("failed to send video packet");
            receive_and_process_decoded_frames(&mut decoder)
                .expect("failed to process video packet");
        }
    }
    decoder.send_eof().expect("failed to send eof");
    receive_and_process_decoded_frames(&mut decoder).expect("failed to eof packet");
}
