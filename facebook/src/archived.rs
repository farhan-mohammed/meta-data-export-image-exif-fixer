// src/my_module.rs
use crate::helpers::*;
use serde::Deserialize;
use serde_json::Error as SerdeJsonError;
use std::fs::metadata;

#[derive(Debug, Deserialize)]
struct MyConfig {
    messages: Vec<MessageItem>,
}

#[derive(Debug, Deserialize)]
struct MessageItem {
    photos: Option<Vec<PhotoItem>>,
    videos: Option<Vec<PhotoItem>>,
    gifs: Option<Vec<PhotoItem>>,
    timestamp_ms: i64,
}

impl MessageItem {
    fn has_media(&self, media: &Option<Vec<PhotoItem>>) -> bool {
        media.is_some() && !media.as_ref().unwrap().is_empty()
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
}

fn process_media_item(medias: &[PhotoItem], src_path: &str, message_timestamp: i64) {
    for media in medias.iter().filter(|media| media.has_uri()) {
        let uri = remove_prefix(
            media.uri.as_ref().unwrap(),
            "your_activity_across_facebook/",
        );
        let photo_file_path = format!("{}/{}", src_path, uri);

        let timestamp = media.timestamp_ms.map_or_else(
            || get_seconds_timestamp(message_timestamp),
            get_seconds_timestamp,
        );

        if let Ok(metadata) = metadata(&photo_file_path) {
            if metadata.is_file() {
                if set_photo_taken_time(&photo_file_path, timestamp).is_ok() {
                    println!("✅ Saved Successfully {} {}", photo_file_path, timestamp);
                } else {
                    println!("❌ Error Saving file {} {}", photo_file_path, timestamp);
                }
            }
        }
    }
}

pub fn read_json_and_get_profile_user(src_path: &str) -> Result<Option<String>, SerdeJsonError> {
    for folder in ["inbox", "e2ee_cutover", "archived_threads"].iter() {
        println!("DOING {} now", folder);
        let directory_path = format!("messages/{}", folder);
        if let Ok(directories) = get_directories(src_path, &directory_path) {
            let total_dir = directories.len();
            for (i, directory) in directories.iter().enumerate() {
                println!(
                    "{}/{} out of {} {}",
                    i + 1,
                    total_dir,
                    directory_path,
                    directory
                );
                let subdirectory_path = format!("{}/{}", directory_path, directory);
                if let Ok(messages_json) =
                    get_json_file_names(src_path, &subdirectory_path, "message")
                {
                    for message_json in messages_json.iter() {
                        let file = get_file(src_path, &subdirectory_path, message_json)
                            .expect("file should open read only");
                        if let Ok(json) = serde_json::from_reader::<_, MyConfig>(file) {
                            let messages = &json.messages;
                            for message in messages.iter() {
                                if message.has_media(&message.photos) {
                                    process_media_item(
                                        message.photos.as_ref().unwrap(),
                                        src_path,
                                        message.timestamp_ms,
                                    );
                                }
                                if message.has_media(&message.videos) {
                                    process_media_item(
                                        message.videos.as_ref().unwrap(),
                                        src_path,
                                        message.timestamp_ms,
                                    );
                                }
                                if message.has_media(&message.gifs) {
                                    process_media_item(
                                        message.gifs.as_ref().unwrap(),
                                        src_path,
                                        message.timestamp_ms,
                                    );
                                }
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
