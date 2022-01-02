
const SEED_SIZE: usize = 48;

pub struct Random {
    data: [u8; SEED_SIZE],
    current_index: usize,
}

impl Random {
    pub fn new(seed: [u8; SEED_SIZE]) -> Self {
        Random {
            data: seed,
            current_index: 0,
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        let first_byte = (self.data[self.current_index] as u32) << 24;
        let second_byte = (self.data[self.current_index + 1] as u32) << 16;
        let third_byte = (self.data[self.current_index + 2] as u32) << 8;
        let fourth_byte = self.data[self.current_index + 3] as u32;

        self.current_index += 4;

        if self.current_index == SEED_SIZE {
            self.shuffle();
            self.current_index = 0;
        }

        first_byte | second_byte | third_byte | fourth_byte
    }

    pub fn next_u32_in_range(&mut self, min: u32, max: u32) -> u32 {
        let rand = self.next_u32();

        min + rand % (max - min)
    }

    pub fn next_usize_in_range(&mut self, min: usize, max: usize) -> usize {
        self.next_u32_in_range(min as u32, max as u32) as usize
    }
    
    // Fake shuffle. Just add numbers to one another, accounting for overflow overflow.
    fn shuffle(&mut self) {
        for i in 0..(self.data.len() - 1) {
            let res: u16 = (self.data[i] as u16) + (self.data[i + 1] as u16) + 1;

            self.data[i] = (res % (u8::MAX as u16 + 1)) as u8;
        }
    }
}