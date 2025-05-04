use crate::{
    brightness::{brightness, brightness_set},
    cli::{Cli, Mode},
    pixels::{pixels, pixels_off},
    text::text,
    transport::Transport,
};
use clap::Parser;
use log::debug;
use servicepoint::{Brightness, HardResetCommand, UdpSocketExt};

mod brightness;
mod cli;
mod image_processing;
mod ledwand_dither;
mod pixels;
mod stream_stdin;
mod stream_window;
mod text;
mod transport;

fn main() {
    let cli = Cli::parse();
    init_logging(cli.verbose);
    debug!("running with arguments: {:?}", cli);

    let transport = Transport::connect(cli.transport, &cli.destination);
    debug!("connection established: {:#?}", transport);

    execute_mode(cli.command, transport);
}

pub fn execute_mode(mode: Mode, connection: Transport) {
    match mode {
        Mode::Reset { force } => {
            if force {
                connection.send_command(HardResetCommand).unwrap()
            } else {
                brightness_set(&connection, Brightness::MAX);
                pixels_off(&connection);
            }
        }
        Mode::Pixels { pixel_command } => pixels(&connection, pixel_command),
        Mode::Brightness { brightness_command } => brightness(&connection, brightness_command),
        Mode::Text { text_command } => text(&connection, text_command),
    }
}

fn init_logging(debug: bool) {
    let filter = if debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    env_logger::builder()
        .filter_level(filter)
        .parse_default_env()
        .init();
}
