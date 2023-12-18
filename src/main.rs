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
        eprintln!(
            "Not enough arguments supplied
Usage: spotify-json <file_path> <Option>
Options:
    -a\t\t\tcalculate total minutes across all artists
    <artist/podcast>\ttype in the name of a artist or show\n",
        );
        exit(1);
    }

    let file_path: &String = &args[1];
    let file_contents: File = match File::open(file_path) {
        Ok(file_contents) => file_contents,
        Err(err) => {
            eprintln!(
                "Failed to open file specified
Usage: spotify-json <file_path> <Option>
Options:
    -a\t\t\tcalculate total minutes across all artists
    <artist/podcast>\ttype in the name of a artist or show\n
ERROR: {}",
                err
            );
            exit(2)
        }
    };

    let buf_reader: BufReader<File> = BufReader::new(file_contents);

    let data: Vec<Data> = match from_reader(buf_reader) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Failed to deserialize from file {}\n{}", file_path, err);
            exit(3)
        }
    };
    let name: &String = &args[2];
    let ms_played: u64 = data
        .iter()
        .filter_map(|d: &Data| {
            if name == "-a" {
                Some(
                    (match d.ms_played {
                        Some(ms_played) => ms_played,
                        None => 0,
                    }) + (match d.ms_played_ext {
                        Some(ms_played_ext) => ms_played_ext,
                        None => 0,
                    }),
                )
            } else if d.artist_name.as_deref() == Some(name) {
                d.ms_played
            } else if d.artist_name_ext.as_deref() == Some(name)
                || d.podcast_name.as_deref() == Some(name)
            {
                d.ms_played_ext
            } else {
                None
            }
        })
        .sum();
    println!(
        "you have listened to {} minutes of {}",
        ms_played / 60000,
        if name == "-a" { "spotify" } else { name }
    );
}
