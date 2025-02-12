use crate::cli::{Cli, Protocol};
use clap::Parser;
use log::debug;
use servicepoint::Connection;

mod cli;
mod execute;
mod stream_stdin;

fn main() {
    let cli = Cli::parse();
    init_logging(cli.verbose);
    debug!("running with arguments: {:?}", cli);

    let connection = make_connection(cli.destination, cli.transport);
    debug!("connection established: {:#?}", connection);

    execute::execute_mode(cli.command, connection);
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
