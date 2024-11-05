use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let _config = match init::check() {
		| Ok(conf) => conf,
		| Err(e) => return Err(e),
	};
	Ok(())
}
