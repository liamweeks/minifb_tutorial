use minifb::{Key, Window, WindowOptions, KeyRepeat};
mod point;
mod renderman;
mod colour;
use crate::colour::Colour;
use crate::renderman::RenderMan;
use crate::point::Point;

use crate::prelude::*;



mod prelude {
    pub const WIDTH: usize= 1280;
    pub const HEIGHT: usize = 740;
    pub use crate::colour::Colour;
    pub use crate::renderman::RenderMan;
    pub use crate::point::Point;
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

    let mut offset_x = 0;
    let mut offset_y = 0;




   /*  while window.is_open() && !window.is_key_down(Key::Escape) {

        
        if window.is_key_down(Key::W) {
            engine.draw_rect(Point::new(offset_x, offset_y), Point::new(40, 40), colours.blue);
            offset_x += 40;
            offset_y += 40;
        } else if window.is_key_down(Key::B) {
            engine.set_background(&colours.green);
        }

        window.update_with_buffer(&engine.buffer, WIDTH, HEIGHT).unwrap()
    } */

    while window.is_open() && !window.is_key_down(Key::Escape) {
        match window.get_keys_pressed(KeyRepeat::No) {
            keys => {
                for key in keys {
                    match key {
                        Key::W => {
                            offset_y -= 40;
                            engine.draw_rect(Point::new(offset_x, offset_y), Point::new(40, 40), colours.blue);
                        }
                        Key::A => {
                            offset_x -= 40;
                            engine.draw_rect(Point::new(offset_x, offset_y), Point::new(40, 40), colours.blue);
                        }
                        Key::S => {
                            offset_y += 40;
                            engine.draw_rect(Point::new(offset_x, offset_y), Point::new(40, 40), colours.blue);
                        }
                        Key::D => {
                            offset_x += 40;
                            engine.draw_rect(Point::new(offset_x, offset_y), Point::new(40, 40), colours.blue);
                        }
                        Key::G => {
                            engine.set_background(&colours.green);
                        }
                        Key::R => {
                            engine.set_background(&colours.red);
                        }
                        Key::B => {
                            engine.set_background(&colours.blue);
                        }
                        _ => {}
                    }
                }
            }
        }
        
        window.update_with_buffer(&engine.buffer, WIDTH, HEIGHT).unwrap();
    }
    
}


