#[derive(Default)]
pub struct Clock(u32); // Defaults to 0

impl Clock {
    pub fn advance_clock(&mut self, cycles: u8) {
        // Learning note: Setting cycles to u8 doesn't save on memory
        // in any meaningful way, but it does indicate that the value
        // of cycles will be a smaller number. u32 would be equally
        // valid.
        self.0 += cycles as u32;
        println!("Clock: {}", self.0);
    }
}
