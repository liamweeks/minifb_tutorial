use minifb::{Key, Window, WindowOptions};
mod point;
mod colours;
use crate::point::Point;
use crate::colours::Colours;

const WIDTH: usize= 640;
const HEIGHT: usize = 360;


fn main() {
    let mut engine: RenderMan = RenderMan::new();

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    // 60 fps cap
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut pixel_x = 100;
    let mut pixel_y = 200;
    let colour = 0xFF0000;

    let mut pixel_index = pixel_y * WIDTH + pixel_x;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        if window.is_key_down(Key::W) {
            engine.draw_rect(Point::new(40, 40), 0xFF0000)
        }        

        window.update_with_buffer(&engine.buffer, WIDTH, HEIGHT).unwrap()
    }
}




pub struct RenderMan {
    pub buffer: Vec<u32>
}

impl RenderMan {
    pub fn colour_point(&mut self, point: Point, colour: u32) {
        self.buffer[((point.x) as u32 + (WIDTH as u32 * (point.y) as u32)) as usize] = colour;
    }

    pub fn draw_rect(&mut self, dimensions: Point, colour: u32) {
        for length in 0..dimensions.x {
            for height in 0..dimensions.y {
                self.colour_point(Point {x: length, y: height }, colour)
            }
        }
    
    }

    pub fn new() -> Self {
        let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        return Self {
            buffer
        };
    }
    
}