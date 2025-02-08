use log::info;
use servicepoint::{Brightness, Command, Connection};
use crate::cli::{BrightnessCommand, Mode, PixelCommand};

pub fn execute_mode(mode: Mode, connection: Connection) {
    match mode {
        Mode::ResetEverything => {
            brightness_reset(&connection);
            pixels_reset(&connection);
        }
        Mode::Pixels { pixel_command } => pixels(&connection, pixel_command),
        Mode::Brightness { brightness_command } => brightness(&connection, brightness_command),
    }
}

fn pixels(connection: &Connection, pixel_command: PixelCommand) {
    match pixel_command {
        PixelCommand::Reset => pixels_reset(&connection),
    }
}

fn brightness(connection: &Connection, brightness_command: BrightnessCommand) {
    match brightness_command {
        BrightnessCommand::Reset => brightness_reset(&connection),
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