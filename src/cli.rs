use clap::{ App, Arg };

pub fn build_cli() -> App<'static, 'static> {
    App::new("sync_files")
        .version("0.1")
        .author("Lilian A. Moraru <lilian.moraru90@gmail.com>")
        .about("")
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("List backups"))
        .arg(Arg::with_name("restore")
            .short("r")
            .long("restore")
            .takes_value(true)
            .value_name("BACKUP")
            .help("Restore a backup on the remote or local destination"))
        .arg(Arg::with_name("clean")
            .short("c")
            .long("clean")
            .help("Clean the backups from the cache"))
        .arg(Arg::with_name("remote")
            .long("remote")
            .takes_value(true)
            .value_name("IP")
            .help("Sync over ssh to a remote target"))
        .arg(Arg::with_name("local")
            .long("local")
            .takes_value(true)
            .value_name("PATH")
            .help("Sync to a local directory"))
        .arg(Arg::with_name("watch")
            .short("w")
            .long("watch")
            .help("Automatically resyncs when the folder has changes"))
}