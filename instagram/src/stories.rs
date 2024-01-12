use crate::helpers::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MyConfig {
    ig_stories: Vec<MediaItem>,
}

#[derive(Debug, Deserialize)]
struct MediaItem {
    uri: String,
    creation_timestamp: i64,
}

pub fn execute(src_path: &str) {
    println!("Started processing stories");

    let json_name = "stories.json";
    let file_path = "/your_instagram_activity/content";

    match get_file(src_path, file_path, json_name) {
        Ok(file) => {
            let json: MyConfig = serde_json::from_reader(file).expect("Unable to read file");
            for media_item in json.ig_stories.iter() {
                let uri = &media_item.uri;
                let photo_file_path = &format!("{src_path}/{uri}");
                let timestamp = get_seconds_timestamp(media_item.creation_timestamp);
                let _ = set_photo_taken_time(photo_file_path, timestamp);
            }
        }
        Err(err) => {
            println!("Error finding/reading {file_path}/{json_name}: {err}");
        }
    }
    println!("Completed Stories.");
}
