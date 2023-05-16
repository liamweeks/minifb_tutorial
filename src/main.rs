use minifb::{Key, Window, WindowOptions};
mod point;
mod renderman;
mod colour;
use crate::colour::Colour;
use crate::renderman::RenderMan;
use crate::point::Point;

use crate::prelude::*;



mod prelude {
    pub const WIDTH: usize= 640;
    pub const HEIGHT: usize = 360;
}


fn main() {
    let mut engine: RenderMan = RenderMan::new();

    let colours = Colour::new();

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    // 60 fps cap
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));




    while window.is_open() && !window.is_key_down(Key::Escape) {

        
        if window.is_key_down(Key::W) {
            engine.draw_rect(Point::new(40, 40), colours.blue);
        } else if window.is_key_down(Key::B) {
            engine.set_background(&colours.green);
        }

        window.update_with_buffer(&engine.buffer, WIDTH, HEIGHT).unwrap()
    }
}


