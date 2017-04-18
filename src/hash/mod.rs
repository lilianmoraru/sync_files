use std::fs::File;
use std::io::Read;
use std::path::Path;

use ssh2::Channel;

error_chain! {
    errors {
        None
    }
}

pub fn local_file_hash(path: &Path) -> String {
    use easy_hash::{ Sha256, Hasher, HashResult };

    // We'll hash the contents of this file
    let mut file = File::open(path).unwrap();

    // Reserve a more appropriate buffer size
    let bytes_len = file.metadata().unwrap().len();
    let mut buffer = Vec::<u8>::with_capacity(bytes_len as usize + 6);

    // Hash the contents
    file.read_to_end(&mut buffer).unwrap();
    let sha256sum = Sha256::hash(&buffer).hex();

    sha256sum
}

pub fn remote_file_hash(ch: &mut Channel, path: &Path) -> String {
    // Construct the command to be executed over SSH
    let mut cmd = String::from("sha256sum ");
    cmd.push_str(path.as_os_str().to_str().unwrap());
    cmd.push_str(" | cut -d ' ' -f 1");

    ch.exec(&cmd).unwrap();

    let mut buffer = String::with_capacity(32);
    ch.read_to_string(&mut buffer).unwrap();

    buffer
}
