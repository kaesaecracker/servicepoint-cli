use crate::cli::{BrightnessCommand, Mode, PixelCommand, StreamCommand};
use crate::stream_stdin::stream_stdin;
use crate::stream_window::stream_window;
use log::info;
use servicepoint::{BitVec, Brightness, Command, CompressionCode, Connection, PIXEL_COUNT};

pub fn execute_mode(mode: Mode, connection: Connection) {
    match mode {
        Mode::ResetEverything => {
            brightness_reset(&connection);
            pixels_reset(&connection);
        }
        Mode::Pixels { pixel_command } => pixels(&connection, pixel_command),
        Mode::Brightness { brightness_command } => brightness(&connection, brightness_command),
        Mode::Stream { stream_command } => match stream_command {
            StreamCommand::Stdin { slow } => stream_stdin(&connection, slow),
            StreamCommand::Screen { options } => stream_window(&connection, options),
        },
    }
}

fn pixels(connection: &Connection, pixel_command: PixelCommand) {
    match pixel_command {
        PixelCommand::Off => pixels_reset(connection),
        PixelCommand::Invert => pixels_invert(connection),
        PixelCommand::On => pixels_on(connection)
    }
}

fn pixels_on(connection: &Connection) {
    let mask = BitVec::repeat(true, PIXEL_COUNT);
    connection
        .send(Command::BitmapLinearXor(0, mask, CompressionCode::Lzma))
        .expect("could not send command")
}

fn pixels_invert(connection: &Connection) {
    let mask = BitVec::repeat(true, PIXEL_COUNT);
    connection
        .send(Command::BitmapLinearXor(0, mask, CompressionCode::Lzma))
        .expect("could not send command")
}

fn brightness(connection: &Connection, brightness_command: BrightnessCommand) {
    match brightness_command {
        BrightnessCommand::Max => brightness_reset(connection),
        BrightnessCommand::Min => brightness_set(connection, Brightness::MIN),
        BrightnessCommand::Set { brightness } => {
            brightness_set(connection, Brightness::saturating_from(brightness))
        }
    }
}

fn pixels_reset(connection: &Connection) {
    connection
        .send(Command::Clear)
        .expect("failed to clear pixels");
    info!("Reset pixels");
}

fn brightness_reset(connection: &Connection) {
    connection
        .send(Command::Brightness(Brightness::MAX))
        .expect("Failed to reset brightness to maximum");
    info!("Reset brightness");
}

fn brightness_set(connection: &Connection, brightness: Brightness) {
    connection
        .send(Command::Brightness(brightness))
        .expect("Failed to set brightness");
    info!("set brightness to {brightness:?}");
}
