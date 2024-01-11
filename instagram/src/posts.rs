// src/my_module.rs
use serde::Deserialize;
use serde_json::Error as SerdeJsonError;
use std::fs::metadata;
use crate::helpers::*;

#[derive(Debug, Deserialize)] // Derive Deserialize trait for your struct
struct MyConfig {
    media: Vec<MediaItem>,
}


#[derive(Debug, Deserialize)]
struct MediaItem {
    uri: String,
    creation_timestamp: i64,
}

pub fn read_json_and_get_profile_user(src_path: &str) -> Result<Option<String>, SerdeJsonError> {
    let directory_path = "/your_instagram_activity/content";
    match get_json_file_names(src_path, directory_path,"post") {
        Ok(posts) => {
            // println!("Posts JSON files in {}: {:?}", directory_path, posts);
            for post in posts.iter(){

                let file = get_file(src_path, directory_path, post).expect("file should open read only");
                let json: MyConfig = serde_json::from_reader(file).expect("Unable to read file");
                let medias = &json.media;
                for media_item in medias.iter() {
                        let uri = &media_item.uri;
                        let photo_file_path = &format!("{src_path}/{uri}");
                        let timestamp = get_seconds_timestamp(media_item.creation_timestamp);
                        if metadata(photo_file_path).is_ok() {
                            let res = set_photo_taken_time(photo_file_path, timestamp);
                            if res.is_ok() {
                                // println!("✅ Saved Successfully {photo_file_path} {timestamp}")
                            }else {
                                // println!("❌ Error Saving file {photo_file_path} {timestamp}");
                            }
                        } 
                    }

            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
    println!("Completed Posts.");
    Ok(None)
}
