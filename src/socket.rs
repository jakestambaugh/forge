use crate::protocol::ForgeRequest;
use lazy_static::lazy_static;
use std::{
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::PathBuf,
};

lazy_static! {
    static ref SOCKET_DIRECTORY: PathBuf = PathBuf::from("/var/run/forge");
    static ref FORGED_SOCKET_PATH: PathBuf = SOCKET_DIRECTORY.join("forged.sock");
}

// Great blog on Unix sockets: https://emmanuelbosquet.com/2022/whatsaunixsocket/

pub fn create_socket_listener() -> std::io::Result<()> {
    std::fs::remove_file(FORGED_SOCKET_PATH.as_path())?;
    let listener = UnixListener::bind(FORGED_SOCKET_PATH.as_path())?;

    loop {
        let (stream, _addr) = listener.accept()?;
        let _ = handle_stream(stream);
    }
}

pub fn send_to_socket(req: &ForgeRequest) -> std::io::Result<()> {
    let mut unix_stream = UnixStream::connect(FORGED_SOCKET_PATH.as_path())?;
    let serialized: String = serde_json::to_string(req).unwrap();

    unix_stream.write(serialized.as_bytes())?;
    Ok(())
}

pub fn handle_stream(mut stream: UnixStream) -> std::io::Result<()> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;

    // TODO: try some of the other parsing styles, like from_reader() or from_bytes()
    let request: ForgeRequest = serde_json::from_str(&buffer).unwrap();

    println!("{:?}", request.command);
    Ok(())
}
