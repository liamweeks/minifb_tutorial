use minifb::{Key, Window, WindowOptions};

const WIDTH: usize= 640;
const HEIGHT: usize = 360;
mod point;
use crate::Point;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

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
        
        

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap()
    }
}


fn colour_point(buffer: &mut Vec<u32>, x: u32, y: u32, colour: u32) {
    buffer[(x + (WIDTH as u32 * y)) as usize] = colour;
}

fn draw_square(buffer: &mut Vec<u32>, x: u32, ) {
    for delta_x in 0..10 {
        for delta_y in 0..10 {
            colour_point(&mut buffer, delta_x, delta_y, 0x00FF00)
        }
    }

}