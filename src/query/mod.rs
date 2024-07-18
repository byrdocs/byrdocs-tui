#[derive(argh::FromArgs,PartialEq,Debug)]
/// Query the metadata
#[argh(subcommand,name="query")]
pub struct Cmd{
	/// specify the kind
	#[argh(option,default="String::new()",short='c')]
	category:String,
	/// the name to be query
	#[argh(option,short='n')]
	name:String,
}
