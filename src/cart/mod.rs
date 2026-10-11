pub mod header;

use anyhow::{Context, Result, anyhow};
use dialoguer::Select;
use std::fs::{self, DirEntry};
use std::{env::current_dir, path::PathBuf};

use crate::cart::header::{CartridgeHeader, licensee::Licensee};

// const MAX_CART_SIZE: usize = 1024 * 1024;
const HEADER_END: usize = 0x150;

// TODO: This will be owned by the MemoryBus struct

pub struct Cartridge {
    pub data: Vec<u8>,
    pub header: CartridgeHeader,
    // loaded: bool,
    // rom_bank: uszie // Determine which ROM bank is available to CPU
    // HM GB has 32 ROM banks (512KiB) and 1 8KiB bank of Cartridge RAM
    // ram: // Not entirely sure how this works yet
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
