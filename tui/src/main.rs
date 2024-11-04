use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	match init::check() {
		| Ok(_) => Ok(()),
		| Err(e) => Err(e),
	}
}
