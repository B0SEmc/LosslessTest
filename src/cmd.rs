use rand::Rng;
use std::fs::remove_file;
use std::path::PathBuf;
use std::process::Command;

use crate::AudioFile;

impl AudioFile {
    pub fn new(file: String, lossless: bool) -> Self {
        Self { file, lossless }
    }
}

fn convert_wav(lossyfile: &str, losslessfile: &str) -> AudioFile {
    println!("Converting {} to WAV", lossyfile);
    let lossless = rand::rng().random_bool(0.5);
    let mut command = Command::new("ffmpeg");
    let name = format!("{}.wav", lossyfile.split('.').collect::<Vec<&str>>()[0]);
    if lossless {
        command.arg("-i").arg(losslessfile).arg(&name);
    } else {
        command.arg("-i").arg(lossyfile).arg(&name);
    }
    command.output().expect("Failed to convert file");
    if !std::path::Path::new(&name).exists() {
        eprintln!("Failed to convert file");
        std::process::exit(1);
    }
    remove_file(&lossyfile).unwrap(); // do NOT remove the lossless file, as it is the original file
    AudioFile::new(name, lossless)
}

pub fn convert_potato(files: Vec<PathBuf>) -> Vec<AudioFile> {
    let mut ret: Vec<AudioFile> = Vec::new();
    for file in files {
        let name = file.file_name().unwrap().to_str().unwrap();
        let lossyfile = format!("{}.mp3", name.strip_suffix(".flac").unwrap());
        Command::new("ffmpeg")
            .arg("-i")
            .arg(&file)
            .arg("-b:a")
            .arg("64k")
            .arg(&lossyfile)
            .output()
            .expect("Failed to convert file");
        let converted = convert_wav(&lossyfile, &name);
        ret.push(converted);
    }
    ret
}

pub fn convert_easy(files: Vec<PathBuf>) -> Vec<AudioFile> {
    let mut ret: Vec<AudioFile> = Vec::new();
    for file in files {
        let name = file.file_name().unwrap().to_str().unwrap();
        let lossyfile = format!("{}.mp3", name.strip_suffix(".flac").unwrap());
        Command::new("ffmpeg")
            .arg("-i")
            .arg(&file)
            .arg("-b:a")
            .arg("128k")
            .arg(&lossyfile)
            .output()
            .expect("Failed to convert file");
        let converted = convert_wav(&lossyfile, &name);
        ret.push(converted);
    }
    ret
}

pub fn convert_medium(files: Vec<PathBuf>) -> Vec<AudioFile> {
    let mut ret: Vec<AudioFile> = Vec::new();
    for file in files {
        let name = file.file_name().unwrap().to_str().unwrap();
        let lossyfile = format!("{}.opus", name.strip_suffix(".flac").unwrap());
        Command::new("ffmpeg")
            .arg("-i")
            .arg(&file)
            .arg("-b:a")
            .arg("128k")
            .arg(&lossyfile)
            .output()
            .expect("Failed to convert file");
        let converted = convert_wav(&lossyfile, &name);
        ret.push(converted);
    }
    ret
}

pub fn convert_hard(files: Vec<PathBuf>) -> Vec<AudioFile> {
    let mut ret: Vec<AudioFile> = Vec::new();
    for file in files {
        let name = file.file_name().unwrap().to_str().unwrap();
        let lossyfile = format!("{}.aac", name.strip_suffix(".flac").unwrap());
        Command::new("ffmpeg")
            .arg("-i")
            .arg(&file)
            .arg("-c:a")
            .arg("aac")
            .arg("-b:a")
            .arg("256k")
            .arg(&lossyfile)
            .output()
            .expect("Failed to convert file");
        let converted = convert_wav(&lossyfile, &name);
        ret.push(converted);
    }
    ret
}
