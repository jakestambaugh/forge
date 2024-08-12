use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Build,
    Test,
}

/// This is the structure of a request message from the CLI to the Server
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    // TODO: consider adding a version field
    pub(crate) command: Command,
    pub(crate) directory: PathBuf,
}
