use std::fs::DirEntry;

use inquire::{InquireError, Select};

#[derive(PartialEq)]
enum Difficulty {
    Potato,
    Easy,
    Medium,
    Hard,
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

fn convert(files: Vec<DirEntry>, difficulty: Difficulty) {}

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
        .collect::<Vec<_>>();
}
