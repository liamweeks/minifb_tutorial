use crate::prelude::*;
use crate::point::Point;
use crate::colour::Colour;

pub struct RenderMan {
    pub buffer: Vec<u32>
}

impl RenderMan {

    pub fn new() -> Self {
        let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        return Self {
            buffer
        };
    }
    
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


    pub fn set_background(&mut self, colour: &u32) {
        for mut pixel in 0..self.buffer.len() {
            self.buffer[pixel] = *colour;
        }
    }
    
}