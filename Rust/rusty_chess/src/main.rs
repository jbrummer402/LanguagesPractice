mod game;
mod board;
use game::Game;

const WIDTH: i32 = 960;
const HEIGHT: i32 = 540;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH,HEIGHT)
        .title("Rusty Chess")
        .build();

    Game::default().run(&mut rl, &thread);
    
}