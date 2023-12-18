use serde::Deserialize;
use serde_json::from_reader;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::process::exit;

#[derive(Deserialize)]
struct Data {
    #[serde(rename = "artistName")]
    artist_name: Option<String>,
    #[serde(rename = "master_metadata_album_artist_name")]
    artist_name_ext: Option<String>,
    #[serde(rename = "episode_show_name")]
    podcast_name: Option<String>,
    #[serde(rename = "msPlayed")]
    ms_played: Option<u64>,
    #[serde(rename = "ms_played")]
    ms_played_ext: Option<u64>,
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 3 {
        eprintln!("Usage: spotify-json <file_path> <artist/podcast_name>");
        exit(1);
    }

    let file_path: &String = &args[1];
    let name: &String = &args[2];
    let file_contents: File = match File::open(file_path) {
        Ok(file_contents) => file_contents,
        Err(err) => {
            eprintln!(
                "Failed to open file specified\nUsage: spotify-json <file_path> <artist_name>\n{}",
                err
            );
            exit(2)
        }
    };

    let buf_reader = BufReader::new(file_contents);

    let data: Vec<Data> = match from_reader(buf_reader) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Failed to deserialize from file {}\n{}", file_path, err);
            exit(3)
        }
    };
    let ms_played: u64 = data
        .iter()
        .filter_map(|d: &Data| {
            if d.artist_name.as_deref() == Some(name) {
                Some(d.ms_played)
            } else if d.artist_name_ext.as_deref() == Some(name) {
                Some(d.ms_played_ext)
            } else if d.podcast_name.as_deref() == Some(name) {
                Some(d.ms_played_ext)
            } else {
                None
            }
        })
        .flatten()
        .sum();
    println!(
        "you have listened to {} minutes of {}",
        ms_played / 60000,
        name
    );
}
