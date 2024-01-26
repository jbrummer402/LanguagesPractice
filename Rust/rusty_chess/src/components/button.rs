use raylib::prelude::*;
use raylib::consts::MouseButton::*;

const MENU_HEIGHT: i32 = 450;
const MENU_WIDTH: i32 = 684;


pub struct Button {
    text: String,
    width: u8,
    height: u8,

}


impl Button {

    fn new(d: &mut RaylibDrawHandle, text: String, width: u8, height: u8) {
        

        Button {
            text: text,
            width: width,
            height: height,
        }
    }

}
