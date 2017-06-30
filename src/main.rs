#[macro_use] extern crate error_chain;
extern crate ring;
extern crate ssh2;
extern crate clap;
#[macro_use] extern crate serde_derive;
extern crate toml;
extern crate globset;
extern crate notify;
extern crate cachedir;
extern crate pbr;

mod config;
mod hash;
mod ssh;
mod cli;
mod errors;

use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    Ok(())
}
