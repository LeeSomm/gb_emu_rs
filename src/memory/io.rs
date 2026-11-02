use crate::memory::timer::Timer;

#[derive(Default)]
pub struct Io {
    pub joypad: u8,             // 0xFF00
    pub sb: u8,                 // 0xFF01 Serial transfer data
    pub sc: u8,                 // 0xFF02 Serial transfer control
    pub timer: Timer,           // 0xFF04-FF07 Timer Registers
    pub int_flag: u8,           // 0xFF0F Interrupt flag
    pub audio: Audio,           // 0xFF10-FF26 Audio
    pub wave_pattern: [u8; 16], // 0xFF30-FF3F Wave Pattern
    pub boot_rom: u8,           // 0xFF50 Boot ROM mapping control
                                // CBG IO Registers are excluded
                                // LCD registers in Ppu struct
}

impl Io {
    pub fn init() -> Self {
        Io {
            joypad: 0,               // 0xFF00
            sb: 0,                   // 0xFF01 Serial transfer data
            sc: 0,                   // 0xFF02 Serial transfer control
            timer: Timer::default(), // 0xFF04-FF07 Timer Registers
            int_flag: 0,             // 0xFF0F Interrupt flag
            audio: Audio::default(), // 0xFF10-FF26 Audio
            wave_pattern: [0; 16],   // 0xFF30-FF3F Wave Pattern
            boot_rom: 0,             // 0xFF50 Boot ROM mapping control
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => self.joypad,
            0xFF01 => self.sb,
            0xFF02 => self.sc,
            0xFF04..=0xFF07 => self.timer.read(addr),
            0xFF0F => self.int_flag,
            0xFF10..=0xFF26 => self.audio.read(addr),
            0xFF30..=0xFF3F => self.wave_pattern[addr as usize - 0xFF30],
            // Gap from LCD registers
            0xFF50 => self.boot_rom,
            _ => 0xFF, // Unused bits of I/O registers return 1.
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF00 => self.joypad = val,
            0xFF01 => self.sb = val,
            0xFF02 => self.sc = val,
            0xFF04..=0xFF07 => self.timer.write(addr, val),
            0xFF0F => self.int_flag = val,
            0xFF10..=0xFF26 => self.audio.write(addr, val),
            0xFF30..=0xFF3F => self.wave_pattern[addr as usize - 0xFF30] = val,
            // Gap from LCD registers
            0xFF50 => self.boot_rom = val,
            _ => (), // NOOP
        }
    }
}

#[derive(Default)]
pub struct Audio {
    pub nr52: u8,           // 0xFF26 Audio master control
    pub nr51: u8,           // 0xFF25 Sound panning
    pub nr50: u8,           // 0xFF24 Master volume & VIN panning
    pub nr10: u8,           // 0xFF10 Channel 1 sweep
    pub nr11: u8,           // 0xFF11 Channel 1 length timer & duty cycle
    pub nr12: u8,           // 0xFF12 Channel 1 volume & envelope
    pub nr13: u8,           // 0xFF13 Channel 1 period low (write-only)
    pub nr14: u8,           // 0xFF14 Channel 1 period high & control
    pub nr21: u8,           // 0xFF16 Channel 2 length timer & duty cycle
    pub nr22: u8,           // 0xFF17 Channel 2 volume & envelope
    pub nr23: u8,           // 0xFF18 Channel 2 period low (write-only)
    pub nr24: u8,           // 0xFF19 Channel 2 period high & control
    pub nr30: u8,           // 0xFF1A Channel 3 DAC enable
    pub nr31: u8,           // 0xFF1B Channel 3 length timer (write-only)
    pub nr32: u8,           // 0xFF1C Channel 3 output level
    pub nr33: u8,           // 0xFF1D Channel 3 period low (write-only)
    pub nr34: u8,           // 0xFF1E Channel 3 period high & control
    pub wave_ram: [u8; 16], // 0xFF30-FF3F Wave pattern RAM
    pub nr41: u8,           // 0xFF20 Channel 4 length timer (write-only)
    pub nr42: u8,           // 0xFF21 Channel 4 volume & envelope
    pub nr43: u8,           // 0xFF22 Channel 4 frequency & randomness
    pub nr44: u8,           // 0xFF23 Channel 4 control
}

impl Audio {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF26 => self.nr52,
            0xFF25 => self.nr51,
            0xFF24 => self.nr50,
            0xFF10 => self.nr10,
            0xFF11 => self.nr11,
            0xFF12 => self.nr12,
            0xFF13 => panic!("Attempted to read write-only audio register NR13"),
            0xFF14 => self.nr14,
            0xFF16 => self.nr21,
            0xFF17 => self.nr22,
            0xFF18 => panic!("Attempted to read write-only audio register NR23"),
            0xFF19 => self.nr24,
            0xFF1A => self.nr30,
            0xFF1B => panic!("Attempted to read write-only audio register NR31"),
            0xFF1C => self.nr32,
            0xFF1D => panic!("Attempted to read write-only audio register NR33"),
            0xFF1E => self.nr34,
            0xFF30..=0xFF3F => self.wave_ram[addr as usize - 0xFF30],
            0xFF20 => panic!("Attempted to read write-only audio register NR41"),
            0xFF21 => self.nr42,
            0xFF22 => self.nr43,
            0xFF23 => self.nr44,
            _ => panic!("Attempted to access invalid audio register: {:#x?}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF26 => self.nr52 = val,
            0xFF25 => self.nr51 = val,
            0xFF24 => self.nr50 = val,
            0xFF10 => self.nr10 = val,
            0xFF11 => self.nr11 = val,
            0xFF12 => self.nr12 = val,
            0xFF13 => self.nr13 = val,
            0xFF14 => self.nr14 = val,
            0xFF16 => self.nr21 = val,
            0xFF17 => self.nr22 = val,
            0xFF18 => self.nr23 = val,
            0xFF19 => self.nr24 = val,
            0xFF1A => self.nr30 = val,
            0xFF1B => self.nr31 = val,
            0xFF1C => self.nr32 = val,
            0xFF1D => self.nr33 = val,
            0xFF1E => self.nr34 = val,
            0xFF30..=0xFF3F => self.wave_ram[addr as usize - 0xFF30] = val,
            0xFF20 => self.nr41 = val,
            0xFF21 => self.nr42 = val,
            0xFF22 => self.nr43 = val,
            0xFF23 => self.nr44 = val,
            _ => panic!("Attempted to write to invalid audio register: {:#x?}", addr),
        }
    }
}
