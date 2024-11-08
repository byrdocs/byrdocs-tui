use std::{error::Error, fs::File, io::Read};

use crypto::{digest::Digest, md5::Md5};

pub fn md5sum(mut file: &File) -> Result<u128, Box<dyn Error>> {
	let mut data = vec![];
	file.read_to_end(&mut data)?;
	println!("{:?}", data);
	let mut md5 = Md5::new();
	md5.input(&data);
	Ok(u128::from_str_radix(md5.result_str().as_str(), 16)?)
}
