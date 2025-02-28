use crate::cli::PixelCommand;
use log::info;
use servicepoint::{BitVec, Command, CompressionCode, Connection, PIXEL_COUNT};

pub(crate) fn pixels(connection: &Connection, pixel_command: PixelCommand) {
    match pixel_command {
        PixelCommand::Off => pixels_off(connection),
        PixelCommand::Invert => pixels_invert(connection),
        PixelCommand::On => pixels_on(connection),
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
