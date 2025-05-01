use crate::{
    cli::{ImageProcessingOptions, PixelCommand, SendImageOptions},
    image_processing::ImageProcessingPipeline,
    stream_window::stream_window,
    transport::Transport,
};
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
