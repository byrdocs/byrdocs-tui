mod cli;
mod lib;
mod add;
mod cover;
mod query;
mod remove;
fn main() {
	let config=lib::config::Config::new();
	let cmd:cli::Cmd=argh::from_env();
	match cmd.nested {
		cli::Subcmd::Add(_) => {
		},
		cli::Subcmd::Cover(_) => {
		},
		cli::Subcmd::Query(_) => {
		},
		cli::Subcmd::Remove(_) => {
		},
	};
}
