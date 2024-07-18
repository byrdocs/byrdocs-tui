#[derive(argh::FromArgs,PartialEq,Debug)]
/// Add new files
#[argh(subcommand,name="add")]
pub struct Cmd{
	/// the path to files
	#[argh(positional)]
	path:std::path::PathBuf,
	#[argh(positional,greedy)]
	paths:Vec<std::path::PathBuf>,
}
