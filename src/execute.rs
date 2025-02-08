use crate::cli::{BrightnessCommand, Mode, PixelCommand};
use log::info;
use servicepoint::{Brightness, Command, Connection};

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
        PixelCommand::Reset => pixels_reset(connection),
    }
}

fn brightness(connection: &Connection, brightness_command: BrightnessCommand) {
    match brightness_command {
        BrightnessCommand::Reset => brightness_reset(connection),
        BrightnessCommand::Min => brightness_set(connection, Brightness::MIN),
        BrightnessCommand::Max => brightness_set(connection, Brightness::MAX),
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
