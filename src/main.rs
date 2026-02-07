use std::io;

use crate::{game::Game, rui::App};
mod game;
mod rui;

fn main() -> io::Result<()> {
    let game = Game::new();

    ratatui::run(|terminal| App::default().run(terminal))
}
