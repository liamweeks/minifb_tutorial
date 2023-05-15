use minifb::{Key, Window, WindowOptions};
mod point;
mod colours;
use crate::point::Point;
use crate::colours::Colours;

const WIDTH: usize= 640;
const HEIGHT: usize = 360;


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
        
        if window.is_key_down(Key::W) {
            draw_rect(&mut buffer, Point::new(40, 40), 0xFF0000)
        }        

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap()
    }
}


fn colour_point(buffer: &mut Vec<u32>, point: Point, colour: u32) {
    buffer[((point.x) as u32 + (WIDTH as u32 * (point.y) as u32)) as usize] = colour;
}

fn draw_rect(buffer: &mut Vec<u32>, point: Point, colour: u32) {
    for length in 0..10 {
        for height in 0..10 {
            colour_point(buffer, Point {x: length, y: height }, colour)
        }
    }

}