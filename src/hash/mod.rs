#![allow(dead_code)] //TODO: Remove this
use std::fs::File;
use std::io::Read;
use std::path::Path;

use ssh2::Channel;

use errors::*;

pub fn local_file_hash(path: &Path) -> Result<String> {
    use ring::digest::{ digest, SHA256 };

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

    // The error message is intended to be somewhat more readable to the user, not to the developer
    // That's why technically, that wasn't the actual issue of the error
    let sha256sum_string = String::from_utf8(sha256sum)
                                  .chain_err(|| {
                                      format!("Failed to obtain the hash for: {}",
                                              path.display())
                                  })?;

    Ok(sha256sum_string)
}

pub fn remote_file_hash(ch: &mut Channel, path: &Path) -> Result<String> {
    // Construct the command to be executed over SSH
    let mut cmd = String::from("sha256sum ");
    let path = path.as_os_str().to_str()
                .ok_or_else(|| ErrorKind::Msg(format!("Failed to obtain the OS string for path: {}",
                                                      path.display()))
                )?;
    cmd.push_str(path);
    cmd.push_str(" | cut -d ' ' -f 1");

    ch.exec(&cmd).chain_err(|| format!("Failed to execute the command: {}", cmd))?;

    let mut buffer = String::with_capacity(32 + 2); // 2: \r\n
    ch.read_to_string(&mut buffer)
      .chain_err(|| format!("Failed to get the result of the command: {}", cmd))?;

    Ok(buffer)
}
