use std::io::{self, Cursor, ErrorKind, Read};

use bytes::Buf;

#[derive(Debug)]
pub struct UF2Block {
	pub start_magic_nums: [u32; 2],
	pub flags: u32,
	pub address: u32,
	pub len: u32,
	pub block_num: u32,
	pub total_blocks: u32,
	// ignoring "File size or board family ID"
	pub data: [u8; 476],
	pub end_magic_num: u32
}

impl UF2Block {
	const CORRECT_START_MAGIC_NUMS: [u32; 2] = [0x0A324655, 0x9E5D5157];
	const CORRECT_END_MAGIC_NUM: u32 = 0x0AB16F30;
	
	// flags
	const NOT_MAIN_FLASH: u32 = 0x00000001;
	const FILE_CONTAINER: u32 = 0x00001000; // block discarded
	// const FAMILY_ID_PRESENT: u32 = 0x00002000; // not implemented
	const MD5_CHECKSUM_PRESENT: u32 = 0x00004000; // ignored
	const EXTENSION_TAGS_PRESENT: u32 = 0x00001000; // ignored

	pub fn verify_magic_nums(&self) -> bool {
		(self.start_magic_nums == Self::CORRECT_START_MAGIC_NUMS)
			&& (self.end_magic_num == Self::CORRECT_END_MAGIC_NUM)
	}

	#[inline]
	pub fn check_flag(&self, flag: u32) -> bool {
		(self.flags & flag) == flag
	}

	#[inline]
	pub fn is_main_flash(&self) -> bool {
		!self.check_flag(Self::NOT_MAIN_FLASH)
	}

	#[inline]
	pub fn is_file_container(&self) -> bool {
		self.check_flag(Self::FILE_CONTAINER)
	}

	#[inline]
	pub fn has_checksum(&self) -> bool {
		self.check_flag(Self::MD5_CHECKSUM_PRESENT)
	}

	#[inline]
	pub fn has_extension_tags(&self) -> bool {
		self.check_flag(Self::EXTENSION_TAGS_PRESENT)
	}

	pub fn contents(&self) -> &[u8] {
		&self.data[0..(self.len as usize)]
	}
}

impl TryFrom<[u8; 512]> for UF2Block {
	type Error = io::Error;

	fn try_from(value: [u8; 512]) -> Result<Self, Self::Error> {
		let mut buf = Cursor::new(value);

		let start_magic_nums = [buf.get_u32_le(), buf.get_u32_le()];
		let flags = buf.get_u32_le();
		let address = buf.get_u32_le();
		let len = buf.get_u32_le();
		let block_num = buf.get_u32_le();
		let total_blocks = buf.get_u32_le();

		buf.set_position(32);
		let mut data = [0; 476];
		buf.read_exact(&mut data)?;

		let end_magic_num = buf.get_u32_le();

		let ans = Self {
			start_magic_nums,
			flags,
			address,
			len,
			block_num,
			total_blocks,
			data,
			end_magic_num
		};

		if ans.verify_magic_nums() {
			Ok(ans)
		} else {
			Err(io::Error::new(ErrorKind::InvalidData, "UF2 magic numbers did not match"))
		}
	}
}
