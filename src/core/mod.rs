use crate::{cart::Cartridge, cpu::Cpu, interrupt::interrupt_service_routine, memory::MemoryBus};

pub struct GameBoy {
    cpu: Cpu,
    bus: MemoryBus,
}

impl GameBoy {
    pub fn init(cart: Cartridge) -> Self {
        GameBoy {
            cpu: Cpu::new(),
            bus: MemoryBus::init(cart),
        }
    }

    pub fn step(&mut self) {
        self.cpu.fetch_and_execute(&mut self.bus);
    }

    pub fn run(&mut self) {
        while 1 == 1 {
            if !self.cpu.halted {
                self.step();
            } else {
                interrupt_service_routine(&mut self.cpu, &mut self.bus);
            }
        }
    }

    pub fn shutdown() {}
}
