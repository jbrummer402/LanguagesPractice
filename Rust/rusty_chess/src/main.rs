use std::fmt::Error;
use rusty_chess::game;

const WIDTH: i32 = 960;
const HEIGHT: i32 = 540;

fn main() -> Result<(), Error> {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH,HEIGHT)
        .title("Rusty Chess")
        .build();

    let mut g = game::Game::default();

    g.run(&mut rl, thread)?;

    Ok(())
}
