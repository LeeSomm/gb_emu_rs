use crate::{
    cart::Cartridge,
    cpu::Cpu,
    interrupt::{get_pending_interrupts, interrupt_service_routine},
    memory::MemoryBus,
    ui::UI,
};

pub struct GameBoy {
    cpu: Cpu,
    bus: MemoryBus,
    ui: UI,
}

impl GameBoy {
    pub fn init(cart: Cartridge) -> Self {
        GameBoy {
            cpu: Cpu::new(),
            bus: MemoryBus::init(cart),
            ui: UI::init(),
        }
    }

    pub fn step(&mut self) {
        self.cpu.fetch_and_execute(&mut self.bus);
    }

    pub fn run(&mut self) {
        // Display debug
        let mut offset = 0;

        loop {
            if self.cpu.halted {
                if get_pending_interrupts(&self.bus).is_some() {
                    // Resume CPU operation if pending interrupts found
                    self.cpu.halted = false;
                } else {
                    // TODO: Logic for non-CPU hardware
                    self.bus.tick(4);
                    continue;
                }
            }

            // Handle interrupts
            if !interrupt_service_routine(&mut self.cpu, &mut self.bus) {
                // Regular CPU operation when no interrupts are handled

                self.step();
            }
            // UI debug
            self.ui.debug(&self.bus, offset);
            // offset = offset.wrapping_add(1);

            self.ui.present();
        }
    }

    pub fn shutdown() {}
}
