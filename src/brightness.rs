use crate::{cli::BrightnessCommand, transport::Transport};
use log::info;
use servicepoint::{Brightness, GlobalBrightnessCommand};

pub(crate) fn brightness(connection: &Transport, brightness_command: BrightnessCommand) {
    match brightness_command {
        BrightnessCommand::Max => brightness_set(connection, Brightness::MAX),
        BrightnessCommand::Min => brightness_set(connection, Brightness::MIN),
        BrightnessCommand::Set { brightness } => {
            brightness_set(connection, Brightness::saturating_from(brightness))
        }
    }
}

pub(crate) fn brightness_set(connection: &Transport, brightness: Brightness) {
    connection
        .send_command(GlobalBrightnessCommand::from(brightness))
        .expect("Failed to set brightness");
    info!("set brightness to {brightness:?}");
}
