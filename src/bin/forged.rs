/// Forge
///
/// New design for a software development environment
///
/// Forge will run as a daemon on the host, and whenever it detects a directory that has a Forgefile,
/// the daemon will subscribe to file system changes.
///
/// Forge CLI will be responsible for dealing with the Forge daemon
///
/// Forgefiles will be YAML with a block for `build`, `test`, and `run`
///
/// forged: the forged  (pronounced "Forge-Dee") scans the filesystem for Forgefiles and marks those directories.
use std::{
    path::{Path, PathBuf},
    thread,
};
use tracing::warn;

use forge::{
    forgefile::{parse_forgefile, Forgefile, FORGEFILE_NAME},
    socket,
};

fn main() -> std::io::Result<()> {
    let socket_listener_handle = thread::spawn(|| socket::create_socket_listener());

    // I'm just now realizing that this code probably has to be multithreaded.
    // There should be a server handling requests from the CLI (maybe multiple CLIs)
    // and there should be another thread scanning the filesystem (or listening for inotify)
    // that runs the subprocesses.

    let paths = scan_for_forgefiles("/home/jake/code");
    let forgefiles: Vec<Forgefile> = paths.iter().map(|dir| parse_forgefile(dir)).collect();
    for f in forgefiles {
        println!("{:?}", f);
    }

    // Once we start running subprocesses, we will want to create a `stdin`, `stdout`, and `stderr` for
    // each subprocess. It probably makes sense to create a socket/file for each in `/var/run/forge` where the
    // naming looks like `<PID>.stdin.sock` and `<PID>.stdout.sock`
    //
    // We will definitely have a map of subprocesses to Forgefiles

    // TODO: I'm pretty sure that `join()` will make this process hang out until the `create_socket_listener`
    // thread returns.
    let _ = socket_listener_handle.join();
    Ok(())
}

fn scan_for_forgefiles<P: AsRef<Path>>(root: P) -> Vec<PathBuf> {
    let exclude: Vec<&Path> = Vec::new();
    scan_for_forgefiles_with_exclusions(root, &exclude)
}

// Call this recursively, don't follow symlinks
// These recursive calls are good candidates for threading to parallelize the reads. Could be as simple as ParIter on read_dir?
fn scan_for_forgefiles_with_exclusions<P: AsRef<Path>>(
    root: P,
    _exclude: &Vec<&Path>,
) -> Vec<PathBuf> {
    let root = root.as_ref();
    let mut forgefile_paths: Vec<PathBuf> = Vec::new();
    // If we aren't starting from a root directory, bail out
    if !root.is_dir() {
        warn!(
            "Attemtped to parse a non-directory path: {}",
            root.display()
        );
        return Vec::new();
    }
    for entry in root.read_dir().expect("read_dir failed") {
        if let Ok(entry) = entry {
            if entry.path().is_file() && entry.file_name() == FORGEFILE_NAME {
                forgefile_paths.push(entry.path());
            }
            if entry.path().is_dir() {
                forgefile_paths.append(&mut scan_for_forgefiles_with_exclusions(
                    entry.path(),
                    &_exclude,
                ))
            }
        }
    }
    forgefile_paths
}

fn _execute_forgefile() {
    unimplemented!()
}

// Useful blog about testing using tempfile: https://andrewra.dev/2019/03/01/testing-in-rust-temporary-files/

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::{create_dir_all, File};

    #[test]
    fn scan_finds_all_0_directories() {
        let tmpdir = tempfile::tempdir().unwrap();
        let found = scan_for_forgefiles(tmpdir);
        assert_eq!(found.len(), 0);
    }

    #[test]
    fn scan_finds_root_directory_with_forgefile() {
        let tmpdir = tempfile::tempdir().unwrap();
        let pathbuf = tmpdir.path().join(FORGEFILE_NAME);
        File::create(tmpdir.path().join(FORGEFILE_NAME)).unwrap();
        let found = scan_for_forgefiles(tmpdir);
        assert_eq!(*found.get(0).unwrap(), pathbuf);
    }

    #[test]
    fn scan_finds_all_subdirectories_with_forgefile() {
        let tmpdir = tempfile::tempdir().unwrap().into_path();
        let pathnames = vec!["a", "a/b", "a/b/c"];
        for &pathname in &pathnames {
            let path = tmpdir.join(pathname);
            create_dir_all(&path).unwrap();
            File::create(path.join(FORGEFILE_NAME)).unwrap();
        }
        let found = scan_for_forgefiles(tmpdir.clone());
        for &pathname in &pathnames {
            assert!(found.contains(&tmpdir.join(pathname).join(FORGEFILE_NAME)));
        }
    }
}
