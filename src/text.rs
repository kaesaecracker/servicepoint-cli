use crate::{cli::TextCommand, stream_stdin::stream_stdin, transport::Transport};

pub fn text(connection: &Transport, command: TextCommand) {
    match command {
        TextCommand::Stdin { slow } => stream_stdin(connection, slow),
    }
}
