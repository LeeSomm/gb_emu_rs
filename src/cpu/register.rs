#[derive(Clone)]
pub enum Register8 {
    A,
    // F is never manipulated directly as an 8bit register
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    // PC is never manipulated directly as a 16bit register
}

pub enum FlagCondition {
    Z,
    NZ,
    C,
    NC,
}

pub struct Register {
    /// Accumulator
    pub a: u8,
    /// Flags (Bytes 7-4)
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    /// Stack Pointer
    pub sp: u16,
    /// Program Counter/Pointer
    pub pc: u16,
}

impl Register {
    pub fn new() -> Self {
        Register {
            a: 0x0,
            f: 0x0,
            b: 0x0,
            c: 0x0,
            d: 0x0,
            e: 0x0,
            h: 0x0,
            l: 0x0,
            sp: 0x0,
            pc: 0x0,
        }
    }

    pub fn debug_print_registers(&self) {
        println!("Register AF: {:#06x?}", self.get_af());
        println!("Register BC: {:#06x?}", self.get_bc());
        println!("Register DE: {:#06x?}", self.get_de());
        println!("Register HL: {:#06x?}", self.get_hl());
        println!("Register SP: {:#06x?}", self.sp);
        println!("Register PC: {:#06x?}", self.pc);
    }

    pub fn reset(&mut self) {
        // After executing boot rom, registers should be set to these values.
        self.set_af(0x01b0); // Maybe the next byte after the cartridge header?
        self.set_bc(0x0013);
        self.set_de(0x00d8);
        self.set_hl(0x014d);
        self.sp = 0xfffe;
        self.pc = 0x0100; // Address of next instruction
    }

    pub fn read_r8(&self, reg: Register8) -> u8 {
        match reg {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::H => self.h,
            Register8::L => self.l,
        }
    }

    pub fn write_r8(&mut self, reg: Register8, val: u8) {
        match reg {
            Register8::A => self.a = val,
            Register8::B => self.b = val,
            Register8::C => self.c = val,
            Register8::D => self.d = val,
            Register8::E => self.e = val,
            Register8::H => self.h = val,
            Register8::L => self.l = val,
        }
    }

    pub fn read_r16(&self, reg: Register16) -> u16 {
        match reg {
            Register16::AF => self.get_af(),
            Register16::BC => self.get_bc(),
            Register16::DE => self.get_de(),
            Register16::HL => self.get_hl(),
            Register16::SP => self.sp,
        }
    }

    pub fn write_r16(&mut self, reg: Register16, val: u16) {
        match reg {
            Register16::AF => self.set_af(val),
            Register16::BC => self.set_bc(val),
            Register16::DE => self.set_de(val),
            Register16::HL => self.set_hl(val),
            Register16::SP => self.sp = val,
        }
    }

    pub fn get_af(&self) -> u16 {
        (u16::from(self.a) << 8) | u16::from(self.f)
    }

    pub fn set_af(&mut self, val: u16) {
        [self.a, self.f] = val.to_be_bytes()
    }

    pub fn get_bc(&self) -> u16 {
        (u16::from(self.b) << 8) | u16::from(self.c)
    }

    pub fn set_bc(&mut self, val: u16) {
        [self.b, self.c] = val.to_be_bytes()
    }

    pub fn get_de(&self) -> u16 {
        (u16::from(self.d) << 8) | u16::from(self.e)
    }

    pub fn set_de(&mut self, val: u16) {
        [self.d, self.e] = val.to_be_bytes()
    }

    pub fn get_hl(&self) -> u16 {
        (u16::from(self.h) << 8) | u16::from(self.l)
    }

    pub fn set_hl(&mut self, val: u16) {
        [self.h, self.l] = val.to_be_bytes();
    }

    pub fn inc_hl(&mut self) {
        let val: u16 = self.get_hl().wrapping_add(1);
        [self.h, self.l] = val.to_be_bytes();
    }

    pub fn dec_hl(&mut self) {
        let val: u16 = self.get_hl().wrapping_sub(1);
        [self.h, self.l] = val.to_be_bytes();
    }

    pub fn clear_flags(&mut self) {
        self.f ^= self.f;
    }

    pub fn get_flag_z_zero(&self) -> u8 {
        // Bit mask for 7th bit
        0b10000000 & self.f
    }

    pub fn set_flag_z_zero(&mut self) {
        self.f |= 0b10000000;
    }

    pub fn clear_flag_z_zero(&mut self) {
        self.f &= 0b01111111;
    }

    pub fn get_flag_n_subtraction(&self) -> u8 {
        // Bit mask for 6th bit
        0b01000000 & self.f
    }

    pub fn set_flag_n_subtraction(&mut self) {
        self.f |= 0b01000000;
    }

    pub fn clear_flag_n_subtraction(&mut self) {
        self.f &= 0b10111111;
    }

    pub fn get_flag_h_half_carry(&self) -> u8 {
        // Bit mask for 5th bit
        0b00100000 & self.f
    }

    pub fn set_flag_h_half_carry(&mut self) {
        self.f |= 0b00100000;
    }

    pub fn clear_flag_h_half_carry(&mut self) {
        self.f &= 0b11011111;
    }

    pub fn get_flag_c_carry(&self) -> u8 {
        // Bit mask for 4th bit
        0b00010000 & self.f
    }

    pub fn set_flag_c_carry(&mut self) {
        self.f |= 0b00010000;
    }

    pub fn clear_flag_c_carry(&mut self) {
        self.f &= 0b11101111;
    }
}
