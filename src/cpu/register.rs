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

    pub fn get_flag_h_halfcarry(&self) -> u8 {
        // Bit mask for 5th bit
        0b00100000 & self.f
    }

    pub fn get_flag_c_carry(&self) -> u8 {
        // Bit mask for 4th bit
        0b00010000 & self.f
    }
}
