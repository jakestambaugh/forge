use lazy_static::lazy_static;
use std::{
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::{Path, PathBuf},
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

pub fn connect_to_socket() -> std::io::Result<()> {
    let mut unix_stream = UnixStream::connect(FORGED_SOCKET_PATH.as_path())?;
    unix_stream.write(b"Hello from the client")?;
    Ok(())
}

pub fn handle_stream(mut stream: UnixStream) -> std::io::Result<()> {
    let mut message = String::new();
    stream.read_to_string(&mut message)?;

    println!("{}", message);
    Ok(())
}
