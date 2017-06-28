#![allow(dead_code)] //TODO: Remove this
use std::fs::File;
use std::io::Read;
use std::path::Path;

use ssh2::Channel;

use errors::*;

pub fn local_file_hash<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    use ring::digest::{ digest, SHA256 };

    let path = path.as_ref();

    // We'll hash the contents of this file
    let mut file = File::open(path)
                    .chain_err(|| format!("Could not open file: {}", path.display()))?;

    // Reserve a more appropriate buffer size
    let bytes_len = file.metadata()
                     .chain_err(|| format!("Could not obtain metadata for: {}", path.display()))?
                     .len();
    let mut buffer = Vec::<u8>::with_capacity(bytes_len as usize + 6);

    // Hash the contents
    file.read_to_end(&mut buffer)
        .chain_err(|| format!("Failed to read from: {}", path.display()))?;

    let sha256sum = digest(&SHA256, &buffer).as_ref().to_vec();

    Ok(sha256sum)
}

pub fn remote_file_hash<P: AsRef<Path>>(ch: &mut Channel, path: P) -> Result<Vec<u8>> {
    let path = path.as_ref();

    // Construct the command to be executed over SSH
    let mut cmd = String::from("sha256sum ");
    let path = path.as_os_str().to_str()
                .ok_or_else(|| ErrorKind::Msg(format!("Failed to obtain the OS string for path: {}",
                                                      path.display()))
                )?;
    cmd.push_str(path);
    cmd.push_str(" | cut -d ' ' -f 1");

    ch.exec(&cmd).chain_err(|| format!("Failed to execute the command: {}", cmd))?;

    //TODO: Remove
    // Leaving these comments in until I get to test the modifications on a target
//    let mut buffer = String::with_capacity(32 + 2); // 2: \r\n
    let mut sha256sum = Vec::<u8>::with_capacity(32 + 2);
    ch.read_to_end(&mut sha256sum)
      .chain_err(|| format!("Failed to get the result of the command: {}", cmd))?;

    //TODO: Remove
    // This also
//    ch.read_to_string(&mut buffer)
//      .chain_err(|| format!("Failed to get the result of the command: {}", cmd))?;

    Ok(sha256sum)
}
