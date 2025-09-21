pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    buffer: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Self {
            buffer: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    pub fn get_buffer(&self) -> &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
        &self.buffer
    }

    pub fn toggle_pixel(&mut self, x: usize, y: usize) -> bool {
        let pixel_on = self.buffer[y][x];

        self.buffer[y][x] ^= true;

        pixel_on
    }
}
