use raylib::prelude::*;
use crate::board::Board;

#[derive(Default)]
pub struct Game {
    player_1_score: u8,
}

impl Game {
    pub fn run(&mut self, mut rl: RaylibHandle, thread: RaylibThread) {
        let mut brd: Board = Board::new();
        let pieces = brd.load_pieces(&mut rl, &thread);

        while !rl.window_should_close() {
            let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
            
            d.clear_background(Color::WHITE);
            d.draw_text(&self.player_1_score.to_string(), 12, 12, 20, Color::BLACK);
            brd.draw(&mut d);

            
            let dest: Rectangle = Rectangle::new(0.0, 0.0, 200.0, 200.0);
            
            // d.draw_texture_pro(
            //     &img_texture,
            //     Rectangle {
            //         x:0.0,
            //         y:0.0,
            //         width: img_texture.width() as f32,
            //         height: img_texture.height() as f32,
            //     },
            //     dest,
            //     Vector2 { x: 0.0, y: 0.0},
            //     0.0,
            //     Color::WHITE,
            // );
            // d.draw_texture(&img_texture, 300, 300, color::Color::WHITE);
            drop(d);
        };
        
    }

    pub fn default() -> Self {
        Self {
            player_1_score: 0,
        }
    }
}