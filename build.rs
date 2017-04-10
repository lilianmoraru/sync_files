extern crate clap;

use clap::Shell;

include!("src/cli.rs");

fn main() {
    let out_dir = ::std::env::var("OUT_DIR").unwrap();

    let mut app = build_cli();
    app.gen_completions("sf", Shell::Bash, &out_dir);
    app.gen_completions("sf", Shell::Zsh, &out_dir);
}
