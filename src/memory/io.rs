pub struct Io {
    pub joypad: u8,   // 0xFF00
    pub sb: u8,       // 0xFF01 Serial transfer data
    pub sc: u8,       // 0xFF02 Serial transfer control
    pub div: u8,      // 0xFF04 Divider register
    pub tima: u8,     // 0xFF05 Timer counter
    pub tma: u8,      // 0xFF06 Timer modulo
    pub tac: u8,      // 0xFF07 Timer control
    pub int_flag: u8, // 0xFF0F Interrupt flag
    pub audio: Audio, // 0xFF10-FF26 Audio
}

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
    pub nr43: u8,           // 0xFF22 Channel 4 Frequency & randomness
    pub nr44: u8,           // 0xFF23 Channel 4 control
}
