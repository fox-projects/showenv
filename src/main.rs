use std::env;

use clap::Parser;

mod cli;

fn main() {
	let _ = cli::Cli::parse();

	for (key, value) in env::vars_os() {
		let key = key.to_str().unwrap();

		println!("{key}: {value:?}");
	}
}
