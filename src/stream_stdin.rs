use crate::transport::Transport;
use log::warn;
use servicepoint::*;
use std::thread::sleep;

pub(crate) fn stream_stdin(connection: &Transport, slow: bool) {
    warn!(
        "This mode will break when using multi-byte characters and does not support ANSI escape sequences yet."
    );
    let mut app = App {
        connection,
        mirror: CharGrid::new(TILE_WIDTH, TILE_HEIGHT),
        y: 0,
        slow,
    };
    app.run()
}

struct App<'t> {
    connection: &'t Transport,
    mirror: CharGrid,
    y: usize,
    slow: bool,
}

impl App<'_> {
    fn run(&mut self) {
        self.connection
            .send_command(ClearCommand)
            .expect("couldn't clear screen");
        let last_y = self.mirror.height() - 1;
        for line in std::io::stdin().lines() {
            let line = line.expect("could not read from stdin");

            if self.y <= last_y {
                self.single_line(&line);
                self.y += 1;
            } else {
                self.shift_rows();
                Self::line_onto_grid(&mut self.mirror, last_y, &line);
                self.send_mirror()
                // we stay on last y
            }

            if self.slow {
                sleep(FRAME_PACING);
            }
        }
    }

    fn shift_rows(&mut self) {
        let data = self.mirror.data_ref_mut();
        data.rotate_left(TILE_WIDTH);
        if let Some(row) = data.last_chunk_mut::<TILE_WIDTH>() {
            row.fill(' ')
        }
    }

    fn line_onto_grid(grid: &mut CharGrid, y: usize, line: &str) {
        for (x, char) in line.chars().enumerate() {
            if x < grid.width() {
                grid.set(x, y, char);
            }
        }
    }

    fn send_mirror(&self) {
        self.connection
            .send_command(CharGridCommand {
                origin: Origin::ZERO,
                grid: self.mirror.clone(),
            })
            .expect("couldn't send screen to display");
    }

    fn single_line(&mut self, line: &str) {
        let mut line_grid = CharGrid::new(TILE_WIDTH, 1);
        line_grid.fill(' ');
        Self::line_onto_grid(&mut line_grid, 0, line);
        Self::line_onto_grid(&mut self.mirror, self.y, line);
        self.connection
            .send_command(CharGridCommand {
                origin: Origin::new(0, self.y),
                grid: line_grid,
            })
            .expect("couldn't send single line to screen");
    }
}
