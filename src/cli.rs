#[derive(argh::FromArgs,PartialEq,Debug)]
/// BYRDOCS Universal Command-line APP
pub struct Cmd{
	#[argh(subcommand)]
	pub nested:Subcmd,
}
#[derive(argh::FromArgs,PartialEq,Debug)]
#[argh(subcommand)]
pub enum Subcmd{
	Add(crate::add::Cmd),
	Cover(crate::cover::Cmd),
	Query(crate::query::Cmd),
	Remove(crate::remove::Cmd),
}
