use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Config {
    default: String,
    sync: HashMap<String, Sync>
}

#[derive(Deserialize, Debug)]
struct Sync {
    files: Option<Vec<String>>,
    dependencies: Option<Vec<String>>,
    symlink_rootfs: Option<String>,
    pre_sync_cmd: Option<String>,
    post_sync_cmd: Option<String>
}
