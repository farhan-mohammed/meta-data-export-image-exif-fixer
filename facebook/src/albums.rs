// src/my_module.rs
use crate::helpers::*;
use serde::Deserialize;
use serde_json::Error as SerdeJsonError;
use std::fs::metadata;
// helpers.rs

#[derive(Debug, Deserialize)] // Derive Deserialize trait for your struct
struct AlbumJson {
    photos: Option<Vec<MediaItem>>,
    cover_photo: Option<MediaItem>,
    last_modified_timestamp: i64,
}
impl AlbumJson {
    fn has_photos(&self) -> bool {
        self.photos.is_some() && !self.photos.as_ref().unwrap().is_empty()
    }
    fn has_cover_photo(&self) -> bool {
        self.cover_photo.is_some()
    }
}

#[derive(Debug, Deserialize)]
struct MediaItem {
    uri: Option<String>,
    creation_timestamp: Option<i64>,
}
impl MediaItem {
    fn has_metadata(&self) -> bool {
        self.uri.is_some() && self.creation_timestamp.is_some()
    }
    fn has_uri(&self) -> bool {
        self.uri.is_some()
    }
}

fn process_media_tem(photo: &MediaItem, src_path: &str, last_modified_timestamp: i64) {
    let uri = remove_prefix(
        photo.uri.as_ref().unwrap(),
        "your_activity_across_facebook/",
    );
    let photo_file_path = &format!("{src_path}/{uri}");
    println!("{}", photo_file_path);
    let mut timestamp = get_seconds_timestamp(last_modified_timestamp);
    if photo.has_metadata() {
        timestamp = get_seconds_timestamp(*photo.creation_timestamp.as_ref().unwrap());
    }
    if metadata(photo_file_path).is_ok() {
        let _ = set_photo_taken_time(photo_file_path, timestamp);
        // if res.is_ok() {
        //     // println!("✅ Saved Successfully {photo_file_path} {timestamp}")
        // } else {
        //     // println!("❌ Error Saving file {photo_file_path} {timestamp}");
        // }
    }
}

pub fn read_json_and_get_profile_user(src_path: &str) -> Result<Option<String>, SerdeJsonError> {
    let subdirectory_path = "posts/album";
    let albums_json = match get_json_file_names(src_path, subdirectory_path, "") {
        Ok(messages_json) => messages_json,
        Err(_err) => return Ok(None),
    };
    for album_json in albums_json.iter() {
        println!("{}", album_json);
        let file =
            get_file(src_path, subdirectory_path, album_json).expect("file should open read only");
        let json: AlbumJson = serde_json::from_reader(file).expect("Unable to read file");
        let last_modified_timestamp = json.last_modified_timestamp;
        if json.has_photos() {
            for photo in json.photos.as_ref().unwrap().iter() {
                if photo.has_uri() {
                    process_media_tem(photo, src_path, last_modified_timestamp)
                }
            }
        }
        if json.has_cover_photo() {
            process_media_tem(
                &json.cover_photo.unwrap(),
                src_path,
                last_modified_timestamp,
            )
        }
    }

    let directory_path = "posts";
    let jsons = match get_json_file_names(src_path, directory_path, "your_posts__check_ins") {
        Ok(messages_json) => messages_json,
        Err(_err) => return Ok(None),
    };
    for json in jsons.iter() {
        println!("{}", json);
        let file = get_file(src_path, subdirectory_path, json).expect("file should open read only");
        let json: AlbumJson = serde_json::from_reader(file).expect("Unable to read file");
        let last_modified_timestamp = json.last_modified_timestamp;
        if json.has_photos() {
            for photo in json.photos.as_ref().unwrap().iter() {
                if photo.has_uri() {
                    process_media_tem(photo, src_path, last_modified_timestamp)
                }
            }
        }
        if json.has_cover_photo() {
            process_media_tem(
                &json.cover_photo.unwrap(),
                src_path,
                last_modified_timestamp,
            )
        }
    }

    println!("Completed Posts.");
    Ok(None)
}
