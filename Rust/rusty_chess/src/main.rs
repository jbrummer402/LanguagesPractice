pub mod components;

use components::menu;
use std::fmt::Error;
use rusty_chess::game;
use raylib::prelude::*;
use raylib::consts::MouseButton::*;

const WIDTH: i32 = 960;
const HEIGHT: i32 = 540;

fn main() -> Result<(), Error> {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH,HEIGHT)
        .title("Rusty Chess")
        .build();

    // let mut game_started = false;
    //
    // if !game_started {
    //     while !(rl.window_should_close()) {
    //
    // let d: &mut RaylibDrawHandle<'_> = &mut rl.begin_drawing(&thread);
    //         d.clear_background(Color::WHITE);
    //         menu::create_menu(d); 
    //     }
    // }
// else if game_started {
        let mut g = game::Game::default();

        g.run(&mut rl, thread)?;
    // }

    Ok(())
}
