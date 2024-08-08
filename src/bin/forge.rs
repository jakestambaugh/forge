/// Forge
///
/// New design for a software development environment
///
/// Forge will run as a daemon on the host, and whenever it detects a directory that has a Forgefile,
/// the daemon will subscribe to file system changes.
///
/// Forge CLI will be responsible for dealing with the Forge daemon
///
/// Forgefiles will be TOML with a block for `build`, `test`, and `run`
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("/var/run/forge/forged.sock")?;
    stream.write_all(b"hello world")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{response}");
    Ok(())
}

// Commands that we want to support:
//
// `status` This should return a list of each of the running subprocesses for the
// tasks configured in the Forgefile here.
