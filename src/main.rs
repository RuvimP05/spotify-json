use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct Data {
    #[serde(rename = "artistName")]
    artist_name: String,
    #[serde(rename = "msPlayed")]
    ms_played: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: spotify-json <file_path>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file_contents: Result<String, std::io::Error> = std::fs::read_to_string(&file_path);
    let file_contents: String = match file_contents {
        Ok(file_contents) => file_contents,
        Err(err) => {
            eprintln!("Error reading file to String: {}", err);
            std::process::exit(1)
        }
    };
    let data: Result<Vec<Data>, serde_json::Error> = serde_json::from_str(file_contents.as_str());
    let data: Vec<Data> = match data {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error deserializing &str: {}", err);
            std::process::exit(1)
        }
    };
    let ms_played: u64 = data
        .iter()
        .filter_map(|d| {
            if d.artist_name == "The Official Podcast" {
                Some(d.ms_played)
            } else {
                None
            }
        })
        .sum();
    println!("{}", ms_played);
}
