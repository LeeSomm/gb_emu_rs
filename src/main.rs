fn main() {
    // See current directory
    if let Ok(path) = std::env::current_dir() {
        println!("{0}", path.display());
    }

    // View header of GB ROM
    if let Ok(rom_file) = std::fs::read("Harvest Moon GB (USA) (SGB Enhanced).gb") {
        let header_start = 0x100;
        let header_end = 0x14F;
        for (i, byte) in rom_file[header_start..=header_end].iter().enumerate() {
            if i % 8 != 7 {
                print!("{:02x} ", byte);
            } else {
                println!("{:02x} ", byte)
            }
        }
        println!();
        // let header_excl_checksum = &rom_file[header_start..header_end - 2];
        let rom_file_excl_checksum = rom_file[..0x14E].iter().chain(rom_file[0x150..].iter());
        // let mut calc_checksum: u16 = 0;
        // for &byte in header_excl_checksum {
        //     calc_checksum = calc_checksum.wrapping_add(byte.into());
        // }
        let calc_checksum =
            rom_file_excl_checksum.fold(0u16, |acc, &byte| acc.wrapping_add(byte.into()));
        println!("Calculated checksum: {:02X?}\n", calc_checksum);
    };

    // let path = gb_emu_rs::cart::cart_choose_rom();
    // println!("{:?}", path);

    // TODO: Find a better way than double unwrap.
    let cart_path = gb_emu_rs::cart::cart_choose_rom().unwrap().unwrap();
    let cart = gb_emu_rs::cart::Cartridge::load(cart_path).unwrap();
    cart.cart_print_info();
    println!();

    // let cpu = gb_emu_rs::cpu::Cpu::new();
    // cpu.register.debug_print_registers();

    let mut gb = gb_emu_rs::core::GameBoy::init(cart);
    gb.run();
}
