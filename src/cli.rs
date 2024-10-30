use std::path::PathBuf;
use std::fs::File;

use clap::Parser;

use uf2_hash::Params;

// todo: change about and stuff (see tutuorial ch1)
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
	/// Flash size in bytes (hexadecimal)
	#[arg(value_name = "FLASH SIZE")]
	flash_size: String,

	/// The path to the UF2 file
	#[arg(value_name = "PATH")]
	path: PathBuf,

	/// Flash start address (hexadecimal)
	#[arg(short, long)]
	start_addr: Option<String>,

	// /// Number of bytes to ignore at the begining of flash. Usefull to avoid hashing the bootrom (hexadecimal)
	// #[arg(short, long)]
	// ignore_size: Option<String>,
}

fn hexstring(s: &str) -> Option<u32> {
	u32::from_str_radix(
		s.strip_prefix("0x")?,
		16
	).ok()
}

impl From<Cli> for Params<File> {
	fn from(value: Cli) -> Self {
		let input_buf = File::open(value.path).unwrap();

		let flash_size = hexstring(&value.flash_size)
			.expect("flash size should be a hexadecimal number starting with 0x");
	
		let start_addr = match value.start_addr {
			Some(addr) => hexstring(&addr).expect("start address should be a hexadecimal number starting with 0x"),
			None => 0,
		};
	
		Self {
			flash_size,
			start_addr,
			input_buf,
		}
	}
}