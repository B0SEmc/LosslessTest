mod cmd;

use cmd::*;
use core::f32;
use inquire::{InquireError, Select};
use open::that;
use std::path::PathBuf;

#[derive(PartialEq)]
enum Difficulty {
    Potato,
    Easy,
    Medium,
    Hard,
}

pub struct AudioFile {
    pub file: String,
    pub lossless: bool,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if &Difficulty::Potato == self {
            write!(f, "Potato (MP3 64kb/s)")
        } else if &Difficulty::Easy == self {
            write!(f, "Easy (MP3 128kb/s)")
        } else if &Difficulty::Medium == self {
            write!(f, "Medium (Opus 128kb/s)")
        } else if &Difficulty::Hard == self {
            write!(f, "Hard (AAC 256kb/s)")
        } else {
            Err(std::fmt::Error)
        }
    }
}

fn convert(files: Vec<PathBuf>, difficulty: Difficulty) -> Vec<AudioFile> {
    match difficulty {
        Difficulty::Potato => convert_potato(files),
        Difficulty::Easy => convert_easy(files),
        Difficulty::Medium => convert_medium(files),
        Difficulty::Hard => convert_hard(files),
    }
}

fn guess(lossless: bool) -> bool {
    match Select::new("Is this file Lossless?", vec![true, false]).prompt() {
        Ok(guess) => guess == lossless,
        Err(InquireError::OperationInterrupted) => std::process::exit(0),
        Err(InquireError::OperationCanceled) => std::process::exit(0),
        Err(_) => {
            println!("Error selecting difficulty");
            std::process::exit(1);
        }
    }
}

fn open_all(files: &Vec<AudioFile>) -> (f32, f32) {
    let mut win: f32 = 0.0;
    let mut loss: f32 = 0.0;
    for file in files {
        that(&file.file).unwrap();
        let guess = guess(file.lossless);
        if guess {
            win += 1.0;
            println!("You guessed right!");
        } else {
            loss += 1.0;
            println!("You guessed wrong!");
        }
    }
    (win, loss)
}

fn main() {
    println!("\x1b[33m𝔏𝔬𝔰𝔰𝔩𝔢𝔰𝔰 𝔗𝔢𝔰𝔱\x1b[0m");
    let difficulty = match Select::new(
        "Select difficulty",
        vec![
            Difficulty::Potato,
            Difficulty::Easy,
            Difficulty::Medium,
            Difficulty::Hard,
        ],
    )
    .prompt()
    {
        Ok(difficulty) => difficulty,
        Err(InquireError::OperationInterrupted) => std::process::exit(0),
        Err(InquireError::OperationCanceled) => std::process::exit(0),
        Err(_) => {
            println!("Error selecting difficulty");
            return;
        }
    };
    println!("Converting FLAC files into lossy formats...");
    let flac_files: Vec<_> = std::fs::read_dir(".")
        .unwrap()
        .filter(|f| {
            let file = f.as_ref().unwrap();
            file.path().extension().is_some() && file.path().extension().unwrap() == "flac"
        })
        .map(|f| {
            let file = f.unwrap();
            file.path()
        })
        .collect::<Vec<_>>();
    if flac_files.is_empty() {
        eprintln!("No FLAC file found in directory!");
        std::process::exit(1);
    }
    let files = convert(flac_files, difficulty);
    let (wins, loss) = open_all(&files);
    files.iter().for_each(|file| {
        std::fs::remove_file(&file.file).ok();
    });
    let ratio = if loss == 0.0 {
        100.0
    } else {
        (100.0 * wins) / loss
    };
    println!(
        "All FLAC files blind tested! Accuracy: {}% ({}x right, {}x false)",
        ratio, wins, loss
    );
}
