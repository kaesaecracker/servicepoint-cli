use servicepoint::{Brightness, Command, Connection};
use log::info;
use crate::cli::BrightnessCommand;

pub(crate) fn brightness(connection: &Connection, brightness_command: BrightnessCommand) {
    match brightness_command {
        BrightnessCommand::Max => brightness_set(connection, Brightness::MAX),
        BrightnessCommand::Min => brightness_set(connection, Brightness::MIN),
        BrightnessCommand::Set { brightness } => {
            brightness_set(connection, Brightness::saturating_from(brightness))
        }
    }
}

pub(crate) fn brightness_set(connection: &Connection, brightness: Brightness) {
    connection
        .send(Command::Brightness(brightness))
        .expect("Failed to set brightness");
    info!("set brightness to {brightness:?}");
}