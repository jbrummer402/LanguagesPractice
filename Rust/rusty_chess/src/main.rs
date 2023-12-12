use std::ffi::{CStr, CString};
use raylib::prelude::*;

const WIDTH: i32 = 960;
const HEIGHT: i32 = 540;

fn main() {
    let (mut rl, thread) = raylib::init()
    .size(WIDTH,HEIGHT)
    .title("Rusty Chess")
    .build();

    let rect1 = Rectangle::new(100.0, 100.0, 50.0, 50.0);
    let check = "Check!".to_string();
    let c_string: CString = CString::new(check.as_str()).unwrap();
    let c_str: &CStr = c_string.as_c_str();


    

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

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
