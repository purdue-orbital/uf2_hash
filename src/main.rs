use std::fs::File;

use clap::Parser;

use anyhow;

use uf2_hash::Params;
use uf2_hash::virtual_flash::VirtualFlash;
use uf2_hash::uf2::UF2;

mod cli;

fn main() -> anyhow::Result<()> {
	let mut params: Params<File> = cli::Cli::parse().into();

	let mut v_flash = VirtualFlash::new_from_params(&params);
	let bin = UF2::parse(params.read())?;
	v_flash.write_uf2(bin)?;

	// println!("{}", bin.blocks.len());
	// println!("{}", bin.blocks[0].total_blocks);
	// println!();
	// for each in bin.blocks {
	// 	println!("{}, {}, {}", each.address, each.len, each.address - v_flash.offset);
	// }

	let hash = v_flash.sha256()?;
	println!("SHA256 hash: {:08x}", hash);
	
	Ok(())
}