use serde::Deserialize;
use serde_json::from_reader;
use std::env::args;
use std::fs::File;
use std::process::exit;

#[derive(Deserialize)]
struct Data {
    #[serde(rename = "artistName")]
    artist_name: String,
    #[serde(rename = "msPlayed")]
    ms_played: u64,
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Usage: spotify-json <file_path>");
        exit(1);
    }

    let file_path: &String = &args[1];
    let file_contents: File = match File::open(file_path) {
        Ok(file_contents) => file_contents,
        Err(err) => {
            eprintln!(
                "Failed to open file specified\nUsage: spotify-json <file_path>\n{}",
                err
            );
            std::process::exit(1)
        }
    };
    let data: Vec<Data> = match from_reader(file_contents) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Failed to deserialize from file {}\n{}", file_path, err);
            exit(2)
        }
    };
    let ms_played: u64 = data
        .iter()
        .filter_map(|d: &Data| {
            if d.artist_name == "The Official Podcast" {
                Some(d.ms_played)
            } else {
                None
            }
        })
        .sum();
    println!("{}", ms_played);
}
