extern crate clap;

mod ssh;
mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();
}
