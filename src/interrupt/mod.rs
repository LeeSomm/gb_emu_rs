/// The Interrupt module exists to provide helper functions for
/// managing system interrupts. The actual data for interrupts
/// spans both the CPU, Memory, and peripherals.
use crate::{cpu::Cpu, memory::MemoryBus};

const IE: u16 = 0xFFFF;
const IF: u16 = 0xFF0F;

// Helper enum for the IF & IE registers
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Interrupt {
    VBlank = 0x40, // Highest priority
    Lcd = 0x48,
    Timer = 0x50,
    Serial = 0x58,
    Joypad = 0x60, // Lowest priority
}

impl Interrupt {
    fn raise(&self, val: u8) -> u8 {
        match self {
            Interrupt::VBlank => val | 0b00000001,
            Interrupt::Lcd => val | 0b00000010,
            Interrupt::Timer => val | 0b00000100,
            Interrupt::Serial => val | 0b00001000,
            Interrupt::Joypad => val | 0b00010000,
        }
    }

    fn clear(&self, val: u8) -> u8 {
        match self {
            Interrupt::VBlank => val & 0b11111110,
            Interrupt::Lcd => val & 0b11111101,
            Interrupt::Timer => val & 0b11111011,
            Interrupt::Serial => val & 0b11110111,
            Interrupt::Joypad => val & 0b11101111,
        }
    }
}

pub fn get_pending_interrupts(bus: &MemoryBus) -> Option<Interrupt> {
    let ie = bus.read(IE);
    let int_flag = bus.read(IF);
    let pending = ie & int_flag & 0x1F; // 0x1F is bit mask for 5 bits
    if pending == 0 {
        None
    } else {
        let bit = pending.trailing_zeros();
        Some(match bit {
            0 => Interrupt::VBlank,
            1 => Interrupt::Lcd,
            2 => Interrupt::Timer,
            3 => Interrupt::Serial,
            4 => Interrupt::Joypad,
            _ => unreachable!(),
        })
    }
}

pub fn interrupt_enable_raise(bus: &mut MemoryBus, interrupt: Interrupt) {
    let ie_val = bus.read(IE);
    bus.write(IE, interrupt.raise(ie_val));
}

pub fn interrupt_flag_raise(bus: &mut MemoryBus, interrupt: Interrupt) {
    let if_val = bus.read(IF);
    bus.write(IF, interrupt.raise(if_val));
}

pub fn interrupt_flag_clear(bus: &mut MemoryBus, interrupt: Interrupt) {
    let if_val = bus.read(IF);
    bus.write(IF, interrupt.clear(if_val));
}

pub fn interrupt_service_routine(cpu: &mut Cpu, bus: &mut MemoryBus) -> bool {
    // If no pending interrupts, nothing happens
    if let Some(interrupt) = get_pending_interrupts(bus) {
        if cpu.ime {
            interrupt_jump_to(cpu, bus, interrupt as u16);
            interrupt_flag_clear(bus, interrupt);
            return true; // Return true to pass that an interrupt was handled.
        }
    }
    false // Return false to pass that no interrupt was handled
}

fn interrupt_jump_to(cpu: &mut Cpu, bus: &mut MemoryBus, addr: u16) {
    cpu.clock.advance_clock(8);
    let [hi, lo] = cpu.register.pc.to_be_bytes();
    cpu.register.sp = cpu.register.sp.wrapping_sub(1);

    cpu.clock.advance_clock(4);
    bus.write(cpu.register.sp, hi);
    cpu.register.sp = cpu.register.sp.wrapping_sub(1);

    cpu.clock.advance_clock(4);
    bus.write(cpu.register.sp, lo);

    cpu.clock.advance_clock(4);
    cpu.register.pc = addr;
    cpu.ime = false;
}
