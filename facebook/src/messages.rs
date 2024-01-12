use crate::helpers::*;
use serde::Deserialize;
use std::io::ErrorKind;

#[derive(Debug, Deserialize)]
struct MyConfig {
    messages: Vec<MessageItem>,
}

#[derive(Debug, Deserialize)]
struct MessageItem {
    photos: Option<Vec<PhotoItem>>,
    videos: Option<Vec<PhotoItem>>,
    audio_files: Option<Vec<PhotoItem>>,
    files: Option<Vec<PhotoItem>>,
    gifs: Option<Vec<PhotoItem>>,
    timestamp_ms: i64,
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

        match set_photo_taken_time(&photo_file_path, timestamp) {
            Ok(_) => 1,
            Err(e) => {
                println!("Error setting time for {photo_file_path} {timestamp} - {e}");
                0
            }
        };
    }
}

pub fn execute(src_path: &str) {
    println!("Starting processing messages.");
    for folder in [
        "inbox",
        "e2ee_cutover",
        "archived_threads",
        "filtered_threads",
        "message_requests",
    ]
    .iter()
    {
        let directory_path = format!("messages/{}", folder);
        println!("processing {directory_path}...");
        match get_directories(src_path, &directory_path) {
            Ok(directories) => {
                if directories.is_empty() {
                    println!(" [skipped: {directory_path} is empty]");
                    continue;
                }
                for directory in directories.iter() {
                    let subdirectory_path = format!("{}/{}", directory_path, directory);
                    if let Ok(messages_json) =
                        get_json_file_names(src_path, &subdirectory_path, "message")
                    {
                        for message_json in messages_json.iter() {
                            let file = get_file(src_path, &subdirectory_path, message_json)
                                .expect("file should open read only");
                            match serde_json::from_reader::<_, MyConfig>(file) {
                                Ok(json) => {
                                    let messages = &json.messages;
                                    for message in messages.iter() {
                                        let ts = message.timestamp_ms;
                                        if let Some(photos) = &message.photos {
                                            process_media_item(photos, src_path, ts);
                                        }
                                        if let Some(videos) = &message.videos {
                                            process_media_item(videos, src_path, ts);
                                        }
                                        if let Some(gifs) = &message.gifs {
                                            process_media_item(gifs, src_path, ts);
                                        }
                                        if let Some(files) = &message.files {
                                            process_media_item(files, src_path, ts);
                                        }
                                        if let Some(audio) = &message.audio_files {
                                            process_media_item(audio, src_path, ts);
                                        }
                                    }
                                }
                                Err(err) => match err.io_error_kind() {
                                    Some(ErrorKind::NotFound) => {
                                        println!(
                                            "Error: File not found: {}/message_json",
                                            &subdirectory_path
                                        );
                                    }
                                    Some(ErrorKind::PermissionDenied) => {
                                        println!(
                                            "Error: Permission denied: {}/message_json",
                                            &subdirectory_path
                                        );
                                    }
                                    _ => println!(
                                        "Error: Error reading {}/{message_json} {err}",
                                        &subdirectory_path
                                    ),
                                },
                            }
                        }
                    }
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => println!("Error: Cannot find Direcotry {directory_path}"),

                ErrorKind::PermissionDenied => {
                    println!("Error: Permission denied accessing: {}]", directory_path);
                }
                _ => println!("Error: Error {err}"),
            },
        }
    }
    println!("Completed Messages.");
}
