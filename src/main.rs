use crate::{
    brightness::{brightness, brightness_set},
    cli::{Cli, Mode, Protocol, StreamCommand},
    pixels::{pixels, pixels_off},
    stream_stdin::stream_stdin,
    stream_window::stream_window,
};
use clap::Parser;
use log::debug;
use servicepoint::{Brightness, Connection};

mod brightness;
mod cli;
mod image_processing;
mod ledwand_dither;
mod pixels;
mod stream_stdin;
mod stream_window;

fn main() {
    let cli = Cli::parse();
    init_logging(cli.verbose);
    debug!("running with arguments: {:?}", cli);

    let connection = make_connection(cli.destination, cli.transport);
    debug!("connection established: {:#?}", connection);

    execute_mode(cli.command, connection);
}

pub fn execute_mode(mode: Mode, connection: Connection) {
    match mode {
        Mode::ResetEverything => {
            brightness_set(&connection, Brightness::MAX);
            pixels_off(&connection);
        }
        Mode::Pixels { pixel_command } => pixels(&connection, pixel_command),
        Mode::Brightness { brightness_command } => brightness(&connection, brightness_command),
        Mode::Stream { stream_command } => match stream_command {
            StreamCommand::Stdin { slow } => stream_stdin(connection, slow),
            StreamCommand::Screen {
                stream_options,
                image_processing,
            } => stream_window(&connection, stream_options, image_processing),
        },
    }
}

fn make_connection(destination: String, transport: Protocol) -> Connection {
    match transport {
        Protocol::Udp => Connection::open(destination).expect("Failed to open UDP connection"),
        Protocol::WebSocket => {
            let url = destination.parse().expect(
                "provided destination is not a valid url - make sure it starts with 'ws://'",
            );
            Connection::open_websocket(url).expect("Failed to open WebSocket connection")
        }
        Protocol::Fake => Connection::Fake,
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
