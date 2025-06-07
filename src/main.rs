use std::path::PathBuf;

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
struct Args {
    from_dir: PathBuf,
    to_dir: PathBuf,
    #[arg(long, default_value_t = Regex::new(r"^..").expect("The default regex value is somehow invalid"))]
    id_regex: Regex,
}

fn main() {
    let x = Args::parse();
    println!("from_dir: {:?}, to_dir: {:?}, id_regex: {:?}", x.from_dir, x.to_dir, x.id_regex);
}
