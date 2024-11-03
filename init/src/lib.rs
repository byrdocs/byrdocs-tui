use std::{
	error::Error,
	path::PathBuf,
};

use check::{check_root_dir,check_subdirctory};
use create::{create_archive_dir, create_cache_dir, create_root_dir, create_stockpile_dir};

pub mod check;
pub mod create;
pub fn check()-> Result<PathBuf, Box<dyn Error>> {
	if let Err(_)=check_root_dir(){
		if let Err(e)=create_root_dir(){
			return Err(e);
		}
	};
	let root_dir=check_root_dir().unwrap();
	if let Err(_)=check_subdirctory(root_dir.clone(),"archive"){
		if let Err(e)=create_archive_dir(root_dir.clone()){
			return Err(e);
		}
	}
	if let Err(_)=check_subdirctory(root_dir.clone(),"cache"){
		if let Err(e)=create_cache_dir(root_dir.clone()){
			return Err(e);
		}
	}
	if let Err(_)=check_subdirctory(root_dir.clone(),"stockpile"){
		if let Err(e)=create_stockpile_dir(root_dir.clone()){
			return Err(e);
		}
	}
	Ok(root_dir)
}
