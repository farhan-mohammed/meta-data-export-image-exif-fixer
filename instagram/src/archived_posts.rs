use crate::helpers::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

pub fn execute(src_path: &str) {
    println!("Started processing archived posts");
    let json_name = "archived_posts.json";
    let file_path = "/your_instagram_activity/content/";

    match get_file(src_path, file_path, json_name) {
        Ok(file) => {
            let json: MyConfig = serde_json::from_reader(file).expect("Unable to read file");

            let media_post = &json.ig_archived_post_media;
            for element in media_post.iter() {
                let medias = &element.media;
                for media_item in medias.iter() {
                    let uri = &media_item.uri;
                    let photo_file_path = &format!("{src_path}/{uri}");
                    let timestamp = get_seconds_timestamp(media_item.creation_timestamp);
                    let _ = set_photo_taken_time(photo_file_path, timestamp);
                }
            }
        }
        Err(err) => {
            println!("[Error reading {}/{} {err}]", file_path, json_name);
        }
    }
    println!("Completed Archived Posts.");
}
