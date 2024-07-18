#[derive(argh::FromArgs,PartialEq,Debug)]
/// Remove files
#[argh(subcommand,name="remove")]
pub struct Cmd{
	/// the path to files
	#[argh(positional)]
	path:std::path::PathBuf,
	#[argh(positional,greedy)]
	paths:Vec<std::path::PathBuf>,
}
