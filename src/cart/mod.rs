pub mod header;

use anyhow::{Context, Result, anyhow};
use dialoguer::Select;
use std::fs::{self, DirEntry};
use std::{env::current_dir, path::PathBuf};

use crate::cart::header::licensee::Licensee;

const MAX_CART_SIZE: usize = 1024 * 1024;
const HEADER_END: usize = 0x150;

pub struct Cartridge {
    pub data: Vec<u8>,
    header: CartridgeHeader,
    // loaded: bool,
}

impl Cartridge {
    pub fn load(path: PathBuf) -> Result<Self> {
        // let rom_file = File::open(path).context("Failed to open ROM file.")?;
        // let reader = BufReader::new(rom_file);

        let data = fs::read(path).context("Failed to read ROM data")?;
        if data.len() >= HEADER_END {
            let header = CartridgeHeader::load(&data);

            Ok(Self {
                data,
                header, // TODO: Method for feeding in header data
            })
        } else {
            Err(anyhow!(
                "Invalid ROM size. ROM must at least be larger than 0x14F (final header address)."
            ))
        }
    }

    pub fn cart_print_info(&self) {
        println!("Entry point: {:02X?}", { self.header.entry_point });
        println!("Title: {:02X?}", {
            String::from_utf8_lossy(&self.header.title)
        });
        println!("CGB Flag: {:02X?}", { self.header.cgb_flag });
        println!(
            "New Licensee Code: {:02X?}\nLicensee: {}",
            self.header.new_licensee_code,
            Licensee::translate_new(self.header.new_licensee_code)
        );
        println!("SGB Flag: {:02X?}", { self.header.sgb_flag });
        println!("Cartridge Type: {:02X?}", { self.header.cartridge_type });
        println!("ROM Size: {:02X?}", { self.header.rom_size });
        println!("RAM Size: {:02X?}", { self.header.ram_size });
        println!("Destination Code: {:02X?}", {
            self.header.destination_code
        });
        println!(
            "Old Licensee Code: {:02X?}\nLicensee: {}",
            self.header.old_licensee_code,
            Licensee::translate_old(self.header.old_licensee_code)
        );
        println!("Mask ROM Version #: {:02X?}", {
            self.header.mask_rom_version_number
        });
        println!("Header Checksum: {:02X?}", { self.header.header_checksum });
        println!("Global Checksum Hi: {:02X?}", {
            self.header.global_checksum_hi
        });
        println!("Global Checksum Lo: {:02X?}", {
            self.header.global_checksum_lo
        });
    }
}

struct CartridgeHeader {
    entry_point: [u8; 4],
    nintendo_logo: [u8; 48],
    title: [u8; 15],
    cgb_flag: u8,
    new_licensee_code: [u8; 2],
    sgb_flag: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination_code: u8,
    old_licensee_code: u8,
    mask_rom_version_number: u8,
    header_checksum: u8,
    global_checksum_hi: u8,
    global_checksum_lo: u8,
}

impl CartridgeHeader {
    fn load(data: &[u8]) -> Self {
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

fn cart_scan_gb_files() -> Result<impl Iterator<Item = DirEntry>> {
    // Get current path of program
    let path: PathBuf = current_dir().context("Invalid working directory.")?;
    // Find entries (files, directories, etc) in current path
    let entries = fs::read_dir(path).context("Invalid directory path.")?;
    // Iterator of all .gb files found in entries
    let gb_files = entries.filter_map(|entry| {
        let entry = entry.ok()?; // Not handling failed reads
        entry
            .path()
            .extension()
            .filter(|ext| *ext == "gb")
            .map(|_| entry)
    });

    Ok(gb_files)
}

pub fn cart_choose_rom() -> Result<Option<PathBuf>> {
    // Text-based I/O to get user to select ROM
    let gb_files = cart_scan_gb_files().context("Failed to scan for Game Boy ROMs")?;
    let gb_files: Vec<DirEntry> = gb_files.collect();

    if gb_files.is_empty() {
        println!("No available ROM files (.gb) were detected.");
        println!("Please make sure ROM files are placed in the working directory.");
        Ok(None)
    } else {
        // println!("Please select a ROM file from the base directory.");
        // println!("Options:");
        // for (i, gbf) in gb_files.iter().enumerate() {
        //     println!("{}. {:?}", i + 1, gbf.file_name())
        // }
        let selection = Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Please select a ROM file from the base directory.")
            .items(
                gb_files
                    .iter()
                    .map(|gbf| gbf.file_name().display().to_string()),
            )
            .default(0)
            .interact_opt()
            .context("Selection of ROM file failed.")?;
        if let Some(i) = selection {
            Ok(Some(gb_files[i].path()))
        } else {
            Ok(None)
        }
    }
}

// pub fn cart_load() -> Result<Cartridge> {
//     let rom_path = cart_choose_rom()?.unwrap();
//     let cart = Cartridge::load(rom_path)?;

//     Ok(cart)
// }
