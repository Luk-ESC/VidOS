mod char;
mod constants;
mod converter;
mod other;

use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::fs::{read_dir, File};
use std::io;
use std::path::PathBuf;

fn main() {
    let _files = get_unconverted_files().expect("io Error. Please report this issue on Github");
    println!("{:?}", _files);
}

fn hashed_filename(path: &PathBuf) -> Result<OsString, io::Error> {
    let mut hasher = Sha256::new();
    io::copy(&mut File::open(path)?, &mut hasher)?;
    Ok(format!("{:x}.data", hasher.finalize()).into())
}

fn get_unconverted_files() -> Result<Vec<(PathBuf, OsString)>, io::Error> {
    let output_videos: Vec<OsString> = read_dir("./videos/out")?
        .map(|entry| Ok::<_, io::Error>(entry?.file_name()))
        .collect::<Result<_, _>>()?;

    let mut input_videos = Vec::new();
    for entry in read_dir("./videos/in")? {
        let path = entry?.path();
        let hash = hashed_filename(&path)?;

        if !output_videos.contains(&hash) {
            input_videos.push((path, hash))
        }
    }

    Ok(input_videos)
}
