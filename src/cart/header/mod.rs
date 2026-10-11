pub mod licensee;

pub struct CartridgeHeader {
    pub entry_point: [u8; 4],
    pub nintendo_logo: [u8; 48],
    pub title: [u8; 15],
    pub cgb_flag: u8,
    pub new_licensee_code: [u8; 2],
    pub sgb_flag: u8,
    pub cartridge_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub destination_code: u8,
    pub old_licensee_code: u8,
    pub mask_rom_version_number: u8,
    pub header_checksum: u8,
    pub global_checksum_hi: u8,
    pub global_checksum_lo: u8,
}

impl CartridgeHeader {
    pub fn load(data: &[u8]) -> Self {
        CartridgeHeader {
            entry_point: bytes(data, 0x100),
            nintendo_logo: bytes(data, 0x104),
            title: bytes(data, 0x134),
            cgb_flag: data[0x143],
            new_licensee_code: bytes(data, 0x144),
            sgb_flag: data[0x146],
            cartridge_type: data[0x147],
            rom_size: data[0x148],
            ram_size: data[0x149],
            destination_code: data[0x14A],
            old_licensee_code: data[0x14B],
            mask_rom_version_number: data[0x14C],
            header_checksum: data[0x14D],
            global_checksum_hi: data[0x14E],
            global_checksum_lo: data[0x14F],
        }
    }
}

/// Helper function for parsing bytes from binary files
fn bytes<const N: usize>(data: &[u8], start: usize) -> [u8; N] {
    data[start..start + N].try_into().unwrap()
}
