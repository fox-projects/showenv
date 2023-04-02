use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
	/// Whether or not you are cool
	#[arg(long)]
	pub isCool: bool,
}
