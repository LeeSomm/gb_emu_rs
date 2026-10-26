#[derive(Default)]
// Clock doubles as the internal divider (u16) used
// by the timer registers. If we eventually need to
// track total t-cycles, we can add a separate u64 value.
pub struct Clock(u16); // Defaults to 0

impl Clock {
    pub fn advance_clock(&mut self, cycles: u16) {
        self.0 = self.0.wrapping_add(cycles);
        // println!("Clock: {}", self.0);
    }
}
