use crate::socket;
use std::process::{Command, Stdio};

/// Create a new subprocess and redirect the output to a Unix socket
pub fn spawn(instruction: &str) {
    let parts: Vec<&str> = instruction.split(" ").collect();
    let output = Command::new(parts.first().unwrap())
        .args(parts.get(1..).unwrap())
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    println!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_starts() {
        let output = Command::new("ls").output().unwrap();
        assert_eq!("Hello", output.stdout.as_slice())
    }
}
