// src/my_module.rs
use crate::helpers::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)] // Derive Deserialize trait for your struct
struct MyConfig {
    media: Vec<MediaItem>,
}

#[derive(Debug, Deserialize)]
struct MediaItem {
    uri: String,
    creation_timestamp: i64,
}

pub fn execute(src_path: &str) {
    println!("Started processing posts");
    let directory_path = "/your_instagram_activity/content";
    match get_json_file_names(src_path, directory_path, "post") {
        Ok(posts) => {
            for post in posts.iter() {
                match get_file(src_path, directory_path, post) {
                    Ok(file) => {
                        let json: MyConfig =
                            serde_json::from_reader(file).expect("Unable to read file");
                        let media_post = &json.media;
                        for media_item in media_post.iter() {
                            let uri = &media_item.uri;
                            let photo_file_path = &format!("{src_path}/{uri}");
                            let timestamp = get_seconds_timestamp(media_item.creation_timestamp);
                            let _ = set_photo_taken_time(photo_file_path, timestamp);
                        }
                    }
                    Err(err) => {
                        println!("[Error reading {directory_path}/{post} {err}]");
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }

    println!("Completed Posts.");
}
