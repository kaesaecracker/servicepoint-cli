use crate::cli::{ImageProcessingOptions, PixelCommand, SendImageOptions};
use crate::image_processing::ImageProcessingPipeline;
use log::info;
use servicepoint::{BitVec, Command, CompressionCode, Connection, Origin, PIXEL_COUNT};

pub(crate) fn pixels(connection: &Connection, pixel_command: PixelCommand) {
    match pixel_command {
        PixelCommand::Off => pixels_off(connection),
        PixelCommand::Flip => pixels_invert(connection),
        PixelCommand::On => pixels_on(connection),
        PixelCommand::Image {
            image_processing_options: processing_options,
            send_image_options: image_options,
        } => pixels_image(connection, image_options, processing_options),
    }
}

fn pixels_on(connection: &Connection) {
    let mask = BitVec::repeat(true, PIXEL_COUNT);
    connection
        .send(Command::BitmapLinear(0, mask, CompressionCode::Lzma))
        .expect("could not send command");
    info!("turned on all pixels")
}

fn pixels_invert(connection: &Connection) {
    let mask = BitVec::repeat(true, PIXEL_COUNT);
    connection
        .send(Command::BitmapLinearXor(0, mask, CompressionCode::Lzma))
        .expect("could not send command");
    info!("inverted all pixels");
}

pub(crate) fn pixels_off(connection: &Connection) {
    connection
        .send(Command::Clear)
        .expect("failed to clear pixels");
    info!("reset pixels");
}

fn pixels_image(
    connection: &Connection,
    options: SendImageOptions,
    processing_options: ImageProcessingOptions,
) {
    let image = image::open(&options.file_name).expect("failed to open image file");
    let pipeline = ImageProcessingPipeline::new(processing_options);
    let bitmap = pipeline.process(image);
    connection
        .send(Command::BitmapLinearWin(
            Origin::ZERO,
            bitmap,
            CompressionCode::default(),
        ))
        .expect("failed to send image command");
    info!("sent image to display");
}
