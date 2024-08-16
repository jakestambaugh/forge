use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// I don't like the Batman-naming of ForgeCommand and ForgeRequest but the terms are so generic that
// it's going to be easier for me this way.

#[derive(Serialize, Deserialize, Debug)]
pub enum ForgeCommand {
    Status,
    // This is a temporary one-shot subprocess run
    Run,
}

/// This is the structure of a request message from the CLI to the Server
#[derive(Serialize, Deserialize, Debug)]
pub struct ForgeRequest {
    // TODO: consider adding a version field
    pub command: ForgeCommand,
    pub directory: PathBuf,
}
