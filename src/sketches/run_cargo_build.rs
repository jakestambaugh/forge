use std::process::{Command, ExitStatus};
use std::thread::sleep;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Start a shell script
    let mut child = Command::new("alphabet.sh").spawn()?;

    let mut status: Option<ExitStatus> = child.try_wait()?;
    while status.is_none() {
        println!("{:?}", child.stdout.as_slice());
        sleep(Duration::from_millis(250));
        status = child.try_wait()?;
    }
    println!("Exit code: {}", status.unwrap().code().unwrap());
    Ok(())
}
