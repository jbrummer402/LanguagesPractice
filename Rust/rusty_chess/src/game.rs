use raylib::prelude::*;
use crate::board;

#[derive(Default)]
pub struct Game {
    player_1_score: u8,
    player_2_score: u8,
}

impl Game {
    pub fn run(&mut self, mut rl: RaylibHandle, thread: RaylibThread) {
        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);
             
            d.clear_background(Color::WHITE);
            d.draw_text(&self.player_1_score.to_string(), 12, 12, 20, Color::BLACK);
            
            // d.gui_check_box(
            //     rect1,
            //     Some(c_str),
            //     false,
            // );
    
            // d.gui_spinner(
            //     rect1,
            //     Some(c_str),
            //     &mut 50,
            //     0,
            //     100,
            //     true,
            // );
        }
        
    }

    pub fn default() -> Self {
        Self {
            player_1_score: 0,
            player_2_score: 0, 
        }
    }
}