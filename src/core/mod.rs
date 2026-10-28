use crate::{
    cart::Cartridge,
    cpu::Cpu,
    display::Display,
    interrupt::{get_pending_interrupts, interrupt_service_routine},
    memory::MemoryBus,
};

pub struct GameBoy {
    cpu: Cpu,
    bus: MemoryBus,
    display: Display,
}

impl GameBoy {
    pub fn init(cart: Cartridge) -> Self {
        GameBoy {
            cpu: Cpu::new(),
            bus: MemoryBus::init(cart),
            display: Display::init(),
        }
    }

    pub fn step(&mut self) {
        self.cpu.fetch_and_execute(&mut self.bus);
    }

    pub fn run(&mut self) {
        loop {
            if self.cpu.halted {
                if get_pending_interrupts(&self.bus).is_some() {
                    // Resume CPU operation if pending interrupts found
                    self.cpu.halted = false;
                } else {
                    // TODO: Logic for non-CPU hardware
                    self.bus.advance_timer(4);
                    continue;
                }
            }

            // Handle interrupts
            if !interrupt_service_routine(&mut self.cpu, &mut self.bus) {
                // Regular CPU operation when no interrupts are handled

                self.step();
            }
            self.display.present();
        }
    }

    pub fn shutdown() {}
}
