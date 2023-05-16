pub struct Colour {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

impl Colour {

    pub fn new() -> Self {
        return Self {
            red: Colour::red(),
            green: Colour::green(),
            blue: Colour::blue(),
        };
    }

    pub fn red() -> u32 {
        return 0xFF0000;
    }
    
    pub fn green() -> u32 {
        return 0x00FF00;
    }

    pub fn blue() -> u32 {
        return 0x0000FF
    }
}