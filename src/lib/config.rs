#[derive(serde::Deserialize)]
pub struct Config{
	pub resources_dir:std::path::PathBuf,
	pub stockpile_dir:std::path::PathBuf,
	pub archive_dir:std::path::PathBuf,
	pub books_dir:std::path::PathBuf,
	pub tests_dir:std::path::PathBuf,
	pub docs_dir:std::path::PathBuf,
	pub covers_dir:std::path::PathBuf,
	pub metadata_path:std::path::PathBuf,
	pub generate_jpg:bool,
	pub generate_png:bool,
	pub generate_webp:bool,
	pub pdf_viewer:String,
	pub zip_viewer:String,
}
fn string2bool(s:&String)->Option<bool>{
	match s.to_lowercase().as_str() {
		"true"=>Some(true),
		"false"=>Some(false),
		_=>None,
	}
}
impl Config{
	pub fn new()->Config{
		let	config_path=home::home_dir()
			.expect("Unable to get home directory")
			.join(".config/byrdocscli.toml");
		let macros=config::Config::builder()
			.add_source(config::File::from(config_path))
			// .add_source(config::Environment::with_prefix("APP"))
			.build()
			.unwrap();
		let mut dic:std::collections::HashMap<String,String>=macros.try_deserialize::<std::collections::HashMap<String,String>>().unwrap();
		for (_key,val) in &mut dic {
			if val.starts_with("~") {
				let new=val.replacen("~", "/home/cpphusky", 1);
				*val=new;
			}
		}
		return Config{
			resources_dir:std::path::PathBuf::from(dic.get("resources_dir").unwrap()),
			stockpile_dir:std::path::PathBuf::from(dic.get("stockpile_dir").unwrap()),
			archive_dir:std::path::PathBuf::from(dic.get("archive_dir").unwrap()),
			books_dir:std::path::PathBuf::from(dic.get("books_dir").unwrap()),
			tests_dir:std::path::PathBuf::from(dic.get("tests_dir").unwrap()),
			docs_dir:std::path::PathBuf::from(dic.get("docs_dir").unwrap()),
			covers_dir:std::path::PathBuf::from(dic.get("covers_dir").unwrap()),
			metadata_path:std::path::PathBuf::from(dic.get("metadata_path").unwrap()),
			generate_jpg:string2bool(dic.get("generate_jpg").unwrap()).unwrap(),
			generate_png:string2bool(dic.get("generate_png").unwrap()).unwrap(),
			generate_webp:string2bool(dic.get("generate_webp").unwrap()).unwrap(),
			pdf_viewer:dic.get("pdf_viewer").unwrap().clone(),
			zip_viewer:dic.get("zip_viewer").unwrap().clone(),
		};
	}
}
