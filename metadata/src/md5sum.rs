use std::{error::Error, fs::File, io::Read};

use crypto::{digest::Digest, md5::Md5};

pub fn md5sum(mut file: File) -> Result<u128, Box<dyn Error>> {
	let mut data = vec![];
	file.read_to_end(&mut data)?;
	println!("{:?}", data);
	let mut md5 = Md5::new();
	md5.input(&data);
	md5.result(&mut data);
	let mut result: u128 = 0;
	for i in 0..16 {
		result = (result << 8) | data[i] as u128;
	}
	Ok(result)
}
