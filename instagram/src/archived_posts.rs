use serde::Deserialize;
use serde_json::Error as SerdeJsonError;
use std::fs::metadata;

use crate::helpers;
use crate::helpers::{get_seconds_timestamp,set_photo_taken_time};

#[derive(Debug, Deserialize)] // Derive Deserialize trait for your struct
struct MyConfig {
    ig_archived_post_media: Vec<MediaContainer>,
}

#[derive(Debug, Deserialize)]
struct MediaContainer {
    media: Vec<MediaItem>,
}

#[derive(Debug, Deserialize)]
struct MediaItem {
    uri: String,
    creation_timestamp: i64,
}

pub fn read_json_and_get_profile_user(src_path: &str) -> Result<Option<String>, SerdeJsonError> {
    let json_name = "archived_posts.json".to_owned();
    let file_path: String = "/your_instagram_activity/content/".to_owned();

    let file = helpers::get_file(src_path,&file_path, &json_name).expect("file should open read only");
    let json: MyConfig = serde_json::from_reader(file).expect("Unable to read file");
   
    let media_post = &json.ig_archived_post_media;

    for element in media_post.iter() {
        let medias = &element.media;
        for media_item in medias.iter() {
            let uri = &media_item.uri;
            let photo_file_path = &format!("{src_path}/{uri}");
            let timestamp = get_seconds_timestamp( media_item.creation_timestamp);
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
    println!("Completed Archived Posts.");
    Ok(None)
}
