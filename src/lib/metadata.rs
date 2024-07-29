use std::io::{Read,Write};
#[derive(serde::Serialize,serde::Deserialize)]
pub struct Books{
	pub md5:String,
	pub name:String,
	pub author:String,
	pub translator:String,
	pub edition:u16,
	pub publisher:String,
	pub isbn:String,
	pub format:String,
	pub size:u64,
}
#[derive(serde::Serialize,serde::Deserialize)]
pub struct Tests{
	pub md5:String,
	pub field:String,
	pub school:String,
	pub term:String,
	pub class:String,
	pub stage:String,
	pub kind:String,
	pub format:String,
	pub size:u64,
}
#[derive(serde::Serialize,serde::Deserialize)]
pub struct Docs{
	pub md5:String,
	pub field:String,
	pub class:String,
	pub name:String,
	pub kind:String,
	pub format:String,
	pub size:u64,
}
#[derive(serde::Serialize,serde::Deserialize)]
pub struct Metadata{
	pub books:Vec<Books>,
	pub tests:Vec<Tests>,
	pub docs:Vec<Docs>,
}
pub fn metadata_get(json_path:std::path::PathBuf)->Result<Metadata,serde_json::Error>{
	let mut file=std::fs::File::open(json_path).unwrap();
	let mut data=String::new();
	file.read_to_string(&mut data).unwrap();
	// file.read_to_string(&mut data).unwrap();
	let metadata:Metadata=serde_json::from_str(&data).expect("JSON was not well-formated");
	return Ok(metadata);
}
pub fn metadata_write(json_path:std::path::PathBuf,metadata:&Metadata)->Result<(),serde_json::Error>{
	let data = serde_json::to_string_pretty(metadata)?;
	let mut file = std::fs::OpenOptions::new()
		.write(true)
		.truncate(true)
		.open(json_path).unwrap();
	file.write_all(data.as_bytes()).unwrap();
	return Ok(());
}
