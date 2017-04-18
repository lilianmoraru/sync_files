#[macro_use] extern crate error_chain;
extern crate easy_hash;
extern crate ssh2;
extern crate clap;
extern crate toml;
extern crate regex;
extern crate notify;
extern crate cachedir;
extern crate pbr;

mod config;
mod hash;
mod ssh;
mod cli;

error_chain! {
    links {
        Config(config::Error, config::ErrorKind);
        Hash(hash::Error, hash::ErrorKind);
        Ssh(ssh::Error, ssh::ErrorKind);
    }
}

quick_main!(run);

fn run() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    Ok(())
}
