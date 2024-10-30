use std::io::{self, Cursor, Read};

pub mod block;

#[derive(Debug)]
pub struct UF2 {
	pub blocks: Vec<block::UF2Block>
}

impl UF2 {
	const BLOCK_SIZE: usize = 512;

	pub fn parse(input: impl AsRef<[u8]>) -> io::Result<UF2> {
		let len = input.as_ref().len();
		let mut blocks = Vec::with_capacity(len / UF2::BLOCK_SIZE);
		let mut buf = Cursor::new(input);
		
		let mut block_bytes = [0_u8; UF2::BLOCK_SIZE];
		while let Ok(_) = buf.read_exact(&mut block_bytes) {
			blocks.push(block_bytes.try_into()?);
		}
		
		
		Ok(Self {
			blocks
		})
	}
}

