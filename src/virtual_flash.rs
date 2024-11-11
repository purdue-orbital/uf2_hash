use std::io::{self, Cursor, IoSlice, IoSliceMut, Read, Write};

use sha2::{Sha256, Digest};

use bytes::Buf;

use crate::Params;
use crate::uf2::block::UF2Block;
use crate::uf2::UF2;

#[derive(Debug)]
pub struct VirtualFlash {
	pub offset: u32,
	pub data: Cursor<Vec<u8>>,
}

impl VirtualFlash {
	// todo? erase is 0s or 1s
	pub fn new(flash_start_addr: u32, size: u32) -> Self {

		Self {
			offset: flash_start_addr,
			data: Cursor::new(vec![0; size as usize]),
		}
	}

	pub fn new_from_params<T>(params: &Params<T>) -> Self {
		Self::new(params.start_addr, params.flash_size)
	}

	#[inline]
	fn address(&self) -> u32 {
		(self.data.position() as u32) + self.offset
	}

	#[inline]
	fn set_address(&mut self, address: u32) {
		self.data.set_position((address - self.offset) as u64);
	}

	#[inline]
	fn seek_start(&mut self) {
		self.set_address(self.offset);
	}

	pub fn into_contents(self) -> Vec<u8> {
		self.data.into_inner()
	}

	pub fn get_contents_ref(&self) -> &Vec<u8> {
		self.data.get_ref()
	}
	
	pub fn get_contents_mut(&mut self) -> &mut Vec<u8> {
		self.data.get_mut()
	}

	pub fn flash_size(&self) -> u32 {
		self.get_contents_ref().len() as u32
	}

	pub fn write_block(&mut self, block: UF2Block) -> io::Result<()> {
		self.set_address(block.address);
		self.write_all(block.contents())?;

		Ok(())
	}

	pub fn write_uf2(&mut self, bin: UF2) -> io::Result<()> {
		for block in bin.blocks {
			self.write_block(block)?;
		}

		Ok(())
	}

	pub fn sha256(&mut self) -> io::Result<[u8; 32]> {
		let mut hasher = Sha256::new();
		self.seek_start();

		// figure out how many 32 bit words we need to read
		let words = self.flash_size() / 4;

		let mut buf = [0; 4]; // 32 bit buffer
		for _ in 0..words {
			self.data.read_exact(&mut buf)?;
			hasher.update(buf);
		}

		for _ in 0..(self.flash_size() % 4) { // hash any remaining bytes
			hasher.update([self.data.get_u8()]);
		}

		let ans = hasher.finalize();
		let ans2 = ans.as_slice();
		// dbg!(&ans2);

		// Ok(ans2)
		Ok(todo!())
	}
}

// TODO!!! when certain read/write trait thingies get stablized, update them here

impl Read for VirtualFlash {
	#[inline]
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.data.read(buf)
	}

	#[inline]
	fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
		self.data.read_vectored(bufs)
	}

	#[inline]
	fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
		self.data.read_exact(buf)
	}

	#[inline]
	fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
		self.data.read_to_end(buf)
	}

	#[inline]
	fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
		self.data.read_to_string(buf)
	}
}

impl Write for VirtualFlash {
	#[inline]
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.data.write(buf)
	}

	#[inline]
	fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
		self.data.write_vectored(bufs)
	}

	#[inline]
	fn flush(&mut self) -> io::Result<()> {
		self.data.flush()
	}
}
