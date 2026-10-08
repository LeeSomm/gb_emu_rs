use std::path::PathBuf;

use crate::{cart::Cartridge, cpu::Cpu};

pub struct Runtime {
    cpu: Cpu,
    cart: Cartridge,
}

impl Runtime {
    pub fn init(cart: Cartridge) -> Self {
        Runtime {
            cpu: Cpu::new(),
            cart,
        }
    }

    pub fn run(&mut self) {
        while 1 == 1 {
            self.cpu.fetch_and_execute();
        }
    }

    pub fn shutdown() {}
}
