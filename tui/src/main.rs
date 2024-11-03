use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let _ = init::check();
	Ok(())
}
