
use raylib::prelude::*;
use raylib::consts::MouseButton::*;

const MENU_HEIGHT: i32 = 450;
const MENU_WIDTH: i32 = 684;


pub fn create_menu(d: &mut RaylibDrawHandle) {

    let menu_rect = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 512.0,
        height: 256.0
    };

    d.draw_rectangle((960 / 2) - (MENU_WIDTH/2), 540/2 - (MENU_HEIGHT/2), MENU_WIDTH, MENU_HEIGHT, Color::SKYBLUE);
    
    d.draw_text("Rusty Chess", MENU_WIDTH/2 - 90, (960 / 2) - (MENU_WIDTH/2), 75, Color::BLACK);

}

pub struct MainMenu {
    
    

}
