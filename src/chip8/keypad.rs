const NUM_KEYS: usize = 16;

pub struct Keypad {
    keys: [bool; NUM_KEYS],
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            keys: [false; NUM_KEYS],
        }
    }

    pub fn set_key_pressed(&mut self, key_index: u8, is_pressed: bool) {
        if (key_index as usize) < NUM_KEYS {
            self.keys[key_index as usize] = is_pressed;
        }
    }

    pub fn is_key_pressed(&self, key_index: u8) -> bool {
        if (key_index as usize) < NUM_KEYS {
            self.keys[key_index as usize]
        } else {
            false
        }
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        for (i, &is_pressed) in self.keys.iter().enumerate() {
            if is_pressed {
                return Some(i as u8);
            }
        }
        None
    }
}
