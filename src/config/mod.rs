use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Config {
    default: String,
    sync: HashMap<String, Sync>,
    target: HashMap<String, Target>
}

#[derive(Deserialize, Debug)]
struct Sync {
    targets: Option<Vec<Target>>,
    files: Option<Vec<String>>,
    dependencies: Option<Vec<String>>,
    symlink_rootfs: Option<String>,
    pre_sync_cmd: Option<String>,
    post_sync_cmd: Option<String>
}

#[derive(Deserialize, Debug)]
struct Target {
    address:  String,
    user:     String,
    password: String
}
