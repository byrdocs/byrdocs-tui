mod add;
mod replace;
#[derive(argh::FromArgs,PartialEq,Debug)]
/// PDF covers editing
#[argh(subcommand,name="cover")]
pub struct Cmd{
	#[argh(subcommand)]
	nested:Subcmd,
}
#[derive(argh::FromArgs,PartialEq,Debug)]
#[argh(subcommand)]
pub enum Subcmd{
	Add(add::Cmd),
	Replace(replace::Cmd),
}
