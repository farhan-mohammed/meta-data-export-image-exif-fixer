// src/my_module.rs
use crate::helpers::*;
use serde::Deserialize;
use serde_json::Error as SerdeJsonError;
use std::fs::metadata;

#[derive(Debug, Deserialize)] // Derive Deserialize trait for your struct
struct MyConfig {
    ig_stories: Vec<MediaItem>,
}

#[derive(Debug, Deserialize)]
struct MediaItem {
    uri: String,
    creation_timestamp: i64,
}

pub fn read_json_and_get_profile_user(src_path: &str) -> Result<Option<String>, SerdeJsonError> {
    let json_name = "stories.json".to_owned();
    let file_path: String = "/your_instagram_activity/content/".to_owned();

    let file = get_file(src_path, &file_path, &json_name);
    match file {
        Ok(file) => {
            let json: MyConfig = serde_json::from_reader(file).expect("Unable to read file");
            let medias = &json.ig_stories;
            for media_item in medias.iter() {

                let uri = &media_item.uri;
                let photo_file_path = &format!("{src_path}/{uri}");
                let timestamp = get_seconds_timestamp( media_item.creation_timestamp);

                // println!("{}", format!("{}", exif_date_time.clone()));
                // println!("{}", format!("{photo_file_path} {timestamp}"));
                if metadata(photo_file_path).is_ok() {
                    let res = set_photo_taken_time(photo_file_path, timestamp);
                    if res.is_ok() {
                        // println!("✅ Saved Successfully {photo_file_path} {timestamp}")
                    }else {
                        // println!("❌ Error Saving file {photo_file_path} {timestamp}");
                    }
                } 
            }

            // Work with the file
            println!("File opened successfully!");
        }
        Err(err) => {
            println!("Error finding/reading {json_name}: {err}");
            // Handle the "not found" error here
        }
    }
    println!("Completed Stories.");

    Ok(None)
}
