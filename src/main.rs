use clap::App;
use clap::load_yaml;
/*
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(short, long)]
	fp: String,
}*/

pub fn main () {
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();

    println!("{:#?}", matches.value_of("filepath"))
}
