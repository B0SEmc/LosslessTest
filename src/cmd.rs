use std::fs::{DirEntry, remove_file};
use std::path::PathBuf;
use std::process::Command;

use crate::AudioFile;

impl AudioFile {
    pub fn new(lossy: String, lossless: String) -> Self {
        Self { lossy, lossless }
    }
}

fn convert_wav(lossyfile: &str, losslessfile: &str) -> AudioFile {
    let mut command = Command::new("ffmpeg");
    let name1 = format!("{}.wav", lossyfile.split('.').collect::<Vec<&str>>()[0]);
    let name2 = format!("{}.wav", losslessfile.split('.').collect::<Vec<&str>>()[0]);
    command
        .arg("-i")
        .arg(lossyfile)
        .arg("-c:a")
        .arg("pcm_s16le")
        .arg(&name1);
    command
        .arg("-i")
        .arg(losslessfile)
        .arg("-c:a")
        .arg("pcm_s16le")
        .arg(&name2);
    remove_file(lossyfile).unwrap(); // do NOT remove the lossless file, as it is the original file
    AudioFile::new(name1, name2)
}

pub fn convert_potato(files: Vec<PathBuf>) -> Vec<AudioFile> {
    let mut ret: Vec<AudioFile> = Vec::new();
    for file in files {
        let name = file.file_name().unwrap().to_str().unwrap();
        let lossyfile = format!("{}.mp3", name);
        let losslessfile = format!("{}.flac", name);
        Command::new("ffmpeg")
            .arg("-i")
            .arg(file)
            .arg("-c:a")
            .arg("libmp3lame")
            .arg("-b:a")
            .arg("64k")
            .arg(&lossyfile)
            .output()
            .expect("Failed to convert file");
        let converted = convert_wav(&lossyfile, &losslessfile);
        ret.push(converted);
    }
    ret
}

pub fn convert_easy(files: Vec<PathBuf>) -> Vec<AudioFile> {
    let mut ret: Vec<AudioFile> = Vec::new();
    for file in files {
        let name = file.file_name().unwrap().to_str().unwrap();
        let lossyfile = format!("{}.mp3", name);
        let losslessfile = format!("{}.flac", name);
        Command::new("ffmpeg")
            .arg("-i")
            .arg(file)
            .arg("-c:a")
            .arg("libmp3lame")
            .arg("-b:a")
            .arg("128k")
            .arg(&lossyfile)
            .output()
            .expect("Failed to convert file");
        let converted = convert_wav(&lossyfile, &losslessfile);
        ret.push(converted);
    }
    ret
}

pub fn convert_medium(files: Vec<PathBuf>) -> Vec<AudioFile> {
    let mut ret: Vec<AudioFile> = Vec::new();
    for file in files {
        let name = file.file_name().unwrap().to_str().unwrap();
        let lossyfile = format!("{}.mp3", name);
        let losslessfile = format!("{}.flac", name);
        Command::new("ffmpeg")
            .arg("-i")
            .arg(file)
            .arg("-c:a")
            .arg("libopus")
            .arg("-b:a")
            .arg("128k")
            .arg(&lossyfile)
            .output()
            .expect("Failed to convert file");
        let converted = convert_wav(&lossyfile, &losslessfile);
        ret.push(converted);
    }
    ret
}

pub fn convert_hard(files: Vec<PathBuf>) -> Vec<AudioFile> {
    let mut ret: Vec<AudioFile> = Vec::new();
    for file in files {
        let name = file.file_name().unwrap().to_str().unwrap();
        let lossyfile = format!("{}.mp3", name);
        let losslessfile = format!("{}.flac", name);
        Command::new("ffmpeg")
            .arg("-i")
            .arg(file)
            .arg("-c:a")
            .arg("aac")
            .arg("-b:a")
            .arg("256k")
            .arg(&lossyfile)
            .output()
            .expect("Failed to convert file");
        let converted = convert_wav(&lossyfile, &losslessfile);
        ret.push(converted);
    }
    ret
}
