use std::io;

use crate::rui::App;
mod game;
mod rui;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
