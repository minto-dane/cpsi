use cps_common::result::ResultExt;
mod cli;
mod database;
mod dependency;
mod package;
mod repository;
mod signature;
mod util;

fn main() {
    let args = std::env::args();
    let packages = args.map(String::from).collect::<Vec<String>>();
    cli::install::install(&packages).unwrap_or_display();
}
