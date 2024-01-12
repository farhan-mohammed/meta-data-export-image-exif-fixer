// src/my_module.rs
use crate::helpers::*;
use serde::Deserialize;
use serde_json::Error as SerdeJsonError;
use std::fs::metadata;

#[derive(Debug, Deserialize)] // Derive Deserialize trait for your struct
struct MyConfig {
    messages: Vec<MessageItem>,
}

#[derive(Debug, Deserialize)]
struct MessageItem {
    photos: Option<Vec<PhotoItem>>,
    videos: Option<Vec<PhotoItem>>,
    timestamp_ms: i64,
}
impl MessageItem {
    fn has_photos(&self) -> bool {
        self.photos.is_some() && !self.photos.as_ref().unwrap().is_empty()
    }
    fn has_videos(&self) -> bool {
        self.videos.is_some() && !self.videos.as_ref().unwrap().is_empty()
    }
}

#[derive(Debug, Deserialize)]
struct PhotoItem {
    uri: Option<String>,
    timestamp_ms: Option<i64>,
}
impl PhotoItem {
    fn has_uri(&self) -> bool {
        self.uri.is_some()
    }
    fn has_timestamp_ms(&self) -> bool {
        self.timestamp_ms.is_some()
    }
}

pub fn read_json_and_get_profile_user(src_path: &str) -> Result<Option<String>, SerdeJsonError> {
    let directory_path = "your_instagram_activity/messages/inbox";
    let directories = match get_directories(src_path, directory_path) {
        Ok(directories) => directories,
        Err(_err) => return Ok(None),
    };
    for directory in directories.iter() {
        let subdirectory_path = format!("{directory_path}/{directory}");
        let messages_json = match get_json_file_names(src_path, &subdirectory_path, "message") {
            Ok(messages_json) => messages_json,
            Err(_err) => return Ok(None),
        };
        // println!("Posts JSON files in {}: {:?}", directory_path, messages_json);
        for message_json in messages_json.iter() {
            let file = get_file(src_path, &subdirectory_path, message_json)
                .expect("file should open read only");
            let json: MyConfig = serde_json::from_reader(file).expect("Unable to read file");

            let messages = &json.messages;
            for message in messages.iter() {
                if message.has_photos() {
                    let photos = message.photos.as_ref().unwrap();
                    for photo in photos.iter() {
                        if !photo.has_uri() {
                            continue;
                        }
                        let uri = photo.uri.as_ref().unwrap();
                        let photo_file_path = &format!("{src_path}/{uri}");

                        let mut timestamp = get_seconds_timestamp(message.timestamp_ms);
                        if photo.has_timestamp_ms() {
                            timestamp =
                                get_seconds_timestamp(*photo.timestamp_ms.as_ref().unwrap());
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
                }

                if message.has_videos() {
                    let videos: &Vec<PhotoItem> = message.videos.as_ref().unwrap();
                    for video in videos.iter() {
                        if !video.has_uri() {
                            continue;
                        }
                        let uri = video.uri.as_ref().unwrap();
                        let photo_file_path = &format!("{src_path}/{uri}");

                        let mut timestamp = get_seconds_timestamp(message.timestamp_ms);
                        if video.has_timestamp_ms() {
                            timestamp =
                                get_seconds_timestamp(*video.timestamp_ms.as_ref().unwrap());
                        }

                        if metadata(photo_file_path).is_ok() {
                            let res = set_photo_taken_time(photo_file_path, timestamp);
                            if res.is_ok() {
                                println!("✅ Saved Successfully {photo_file_path} {timestamp}")
                            } else {
                                println!("❌ Error Saving file {photo_file_path} {timestamp}");
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Completed Messages.");
    Ok(None)
}
