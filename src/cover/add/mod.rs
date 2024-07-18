#[derive(argh::FromArgs,PartialEq,Debug)]
/// Add cover to pdf
#[argh(subcommand,name="add")]
pub struct Cmd{
	/// the path to cover
	#[argh(positional)]
	cover:std::path::PathBuf,
	/// the path to pdf
	#[argh(positional)]
	pdf:std::path::PathBuf,
}
