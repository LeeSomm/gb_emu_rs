#[derive(Default)]
pub struct Clock(u32); // Defaults to 0

impl Clock {
    pub fn advance_clock(&mut self, cycles: u32) {
        self.0 = self.0.wrapping_add(cycles);
        // println!("Clock: {}", self.0);
    }
}
