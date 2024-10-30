use std::{fs::File, io::Read};

pub mod virtual_flash;
pub mod uf2;

#[derive(Debug)]
pub struct Params<T> {
	pub flash_size: u32,
	pub start_addr: u32,	
	pub input_buf: T,
}

// TODO clean up
impl Params<File> {
	pub fn read(&mut self) -> Vec<u8> {
		let mut buf = Vec::new();

		self.input_buf.read_to_end(&mut buf).unwrap();
		
		buf
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn it_works() {
// 		let result = add(2, 2);
// 		assert_eq!(result, 4);
// 	}
// }
