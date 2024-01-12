use crate::helpers::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AlbumJson {
    photos: Option<Vec<MediaItem>>,
    cover_photo: Option<MediaItem>,
    last_modified_timestamp: i64,
}

#[derive(Debug, Deserialize)]
struct MediaItem {
    uri: Option<String>,
    creation_timestamp: Option<i64>,
}
impl MediaItem {
    fn has_uri(&self) -> bool {
        self.uri.is_some()
    }
}

fn process_media_tem(photo: &MediaItem, src_path: &str, last_modified_timestamp: i64) {
    let uri = remove_prefix(
        photo.uri.as_ref().unwrap(),
        "your_activity_across_facebook/",
    );
    let photo_file_path: String = format!("{}/{}", src_path, uri);
    let timestamp = photo.creation_timestamp.map_or_else(
        || get_seconds_timestamp(last_modified_timestamp),
        get_seconds_timestamp,
    );
    match set_photo_taken_time(&photo_file_path, timestamp) {
        Ok(_) => 1,
        Err(e) => {
            println!("Error setting time for {photo_file_path} {timestamp} - {e}");
            0
        }
    };
}

pub fn execute(src_path: &str) {
    println!("Starting processing posts/album.");
    let subdirectory_path = "posts/album";
    let albums_json = match get_json_file_names(src_path, subdirectory_path, "") {
        Ok(messages_json) => messages_json,
        Err(_err) => [].to_vec(),
    };
    for album_json in albums_json.iter() {
        match get_file(src_path, subdirectory_path, album_json) {
            Ok(file) => {
                let json: AlbumJson = serde_json::from_reader(file).expect("Unable to read file");
                let last_modified_timestamp = json.last_modified_timestamp;
                if let Some(photos) = json.photos {
                    for photo in photos.iter() {
                        if photo.has_uri() {
                            process_media_tem(photo, src_path, last_modified_timestamp)
                        }
                    }
                }
                if let Some(cover_photo) = json.cover_photo {
                    process_media_tem(&cover_photo, src_path, last_modified_timestamp)
                }
            }
            Err(err) => {
                println!("[Error reading {}/{} {err}]", subdirectory_path, album_json);
            }
        };
    }
    println!("Completed posts/album.");
}
