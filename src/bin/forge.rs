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
use forge::socket;

fn main() -> std::io::Result<()> {
    socket::connect_to_socket()?;
    Ok(())
}

// Commands that we want to support:
//
// `status` This should return a list of each of the running subprocesses for the
// tasks configured in the Forgefile here.

// Helpful blog post about managing stdout: https://andrewra.dev/2019/08/05/testing-in-rust-writing-to-stdout/
