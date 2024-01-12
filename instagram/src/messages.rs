// src/my_module.rs
use crate::helpers::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MyConfig {
    messages: Vec<MessageItem>,
}

#[derive(Debug, Deserialize)]
struct MessageItem {
    photos: Option<Vec<MediaItem>>,
    videos: Option<Vec<MediaItem>>,
    timestamp_ms: i64,
}

#[derive(Debug, Deserialize)]
struct MediaItem {
    uri: Option<String>,
    timestamp_ms: Option<i64>,
}
impl MediaItem {
    fn has_uri(&self) -> bool {
        self.uri.is_some()
    }
}

fn process_media(media: &MediaItem, src_path: &str, message_ts: i64) {
    let uri = media.uri.as_ref().unwrap();
    if uri.contains("http") {
        return;
    }
    let media_file_path = &format!("{src_path}/{uri}");
    let timestamp = media
        .timestamp_ms
        .map_or_else(|| get_seconds_timestamp(message_ts), get_seconds_timestamp);
    let _ = set_photo_taken_time(media_file_path, timestamp);
}

pub fn execute(src_path: &str) {
    println!("Starting processing messages...");
    let directory_path = "your_instagram_activity/messages/inbox";
    match get_directories(src_path, directory_path) {
        Ok(directories) => {
            for directory in directories.iter() {
                let subdirectory_path = format!("{directory_path}/{directory}");

                match get_json_file_names(src_path, &subdirectory_path, "message") {
                    Ok(messages_json) => {
                        for message_json in messages_json.iter() {
                            let file = get_file(src_path, &subdirectory_path, message_json)
                                .expect("file should open read only");
                            let json: MyConfig =
                                serde_json::from_reader(file).expect("Unable to read file");
                            let messages = &json.messages;

                            for message in messages.iter() {
                                let message_timestamp = message.timestamp_ms;
                                if let Some(photos) = &message.photos {
                                    for photo in photos.iter() {
                                        if photo.has_uri() {
                                            process_media(photo, src_path, message_timestamp);
                                        }
                                    }
                                }
                                if let Some(videos) = &message.videos {
                                    for video in videos.iter() {
                                        if video.has_uri() {
                                            process_media(video, src_path, message_timestamp);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        println!("Error getting json files {directory_path}: {err}");
                    }
                };
            }
        }
        Err(err) => {
            println!("Error getting directories {directory_path}: {err}");
        }
    }
    println!("Completed Messages.");
}
