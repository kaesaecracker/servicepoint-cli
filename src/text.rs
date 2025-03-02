use servicepoint::Connection;
use crate::cli::TextCommand;
use crate::stream_stdin::stream_stdin;

pub fn text(connection: &Connection, command: TextCommand) {
   match command { TextCommand::Stdin  { slow } => stream_stdin(connection, slow), }
}
