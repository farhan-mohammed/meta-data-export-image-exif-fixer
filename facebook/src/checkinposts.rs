// src/my_module.rs
use crate::helpers::*;
use serde::Deserialize;
use serde_json::Error as SerdeJsonError;
use std::fs::metadata;
// helpers.rs

#[derive(Debug, Deserialize)]
struct CheckInPosts {
    attachments: Option<Vec<CheckInMedia>>,
}
impl CheckInPosts {
    fn has_attachments(&self) -> bool {
        self.attachments.is_some()
    }
}

#[derive(Debug, Deserialize)]
struct CheckInMedia {
    data: Option<Vec<Media>>,
}
impl CheckInMedia {
    fn has_data(&self) -> bool {
        self.data.is_some()
    }
}

#[derive(Debug, Deserialize)]
struct Media {
    media: Option<MediaItem>,
}
impl Media {
    fn has_media(&self) -> bool {
        self.media.is_some()
    }
    fn get_media(&self) -> &MediaItem {
        self.media.as_ref().unwrap()
    }
}

#[derive(Debug, Deserialize)]
struct MediaItem {
    uri: Option<String>,
    creation_timestamp: Option<i64>,
    media_metadata: Option<MediaMetadata>,
}
impl MediaItem {
    fn has_uri(&self) -> bool {
        self.uri.is_some()
    }
    fn has_creation_timestamp(&self) -> bool {
        self.creation_timestamp.is_some()
    }
    fn has_media_metadata(&self) -> bool {
        self.media_metadata.is_some()
    }
}

#[derive(Debug, Deserialize)]
struct MediaMetadata {
    photo_metadata: Option<PhotoMetadata>,
}
impl MediaMetadata {
    fn has_metadata(&self) -> bool {
        self.photo_metadata.is_some()
    }
}
#[derive(Debug, Deserialize)]
struct PhotoMetadata {
    exif_data: Option<Vec<ExifData>>,
}
impl PhotoMetadata {
    fn has_exif_data(&self) -> bool {
        self.exif_data.is_some()
    }
}

#[derive(Debug, Deserialize)]
struct ExifData {
    taken_timestamp: Option<i64>,
}
impl ExifData {
    fn has_taken_timestamp(&self) -> bool {
        self.taken_timestamp.is_some()
    }
}

fn process_media_tem(photo_uri: &str, src_path: &str, last_modified_timestamp: i64) {
    let uri = remove_prefix(photo_uri, "your_activity_across_facebook/");
    let photo_file_path = &format!("{src_path}/{uri}");
    println!("{}", photo_file_path);
    let timestamp = get_seconds_timestamp(last_modified_timestamp);
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
    let directory_path = "posts";
    let jsons = match get_json_file_names(src_path, directory_path, "your_posts__check_ins") {
        Ok(messages_json) => messages_json,
        Err(_err) => return Ok(None),
    };
    for json in jsons.iter() {
        println!("{}", json);
        let file = get_file(src_path, directory_path, json).expect("file should open read only");
        let json: Vec<CheckInPosts> = serde_json::from_reader(file).expect("Unable to read file");
        for media in json {
            if media.has_attachments() {
                let attachments = media.attachments.as_ref().unwrap();
                for checkin_media in attachments.iter() {
                    if checkin_media.has_data() {
                        let data = checkin_media.data.as_ref().unwrap();
                        for d in data.iter() {
                            if d.has_media() {
                                let media = d.get_media();
                                if media.has_uri() {
                                    let photo_uri = media.uri.as_ref().unwrap();
                                    println!("{photo_uri}");
                                    if media.has_media_metadata() {
                                        let media_metadata = media.media_metadata.as_ref().unwrap();
                                        if media_metadata.has_metadata() {
                                            let metadata =
                                                media_metadata.photo_metadata.as_ref().unwrap();
                                            if metadata.has_exif_data() {
                                                let exif_data =
                                                    metadata.exif_data.as_ref().unwrap();
                                                let mut timestamp: Option<i64> = None;
                                                for e in exif_data.iter() {
                                                    if e.has_taken_timestamp() {
                                                        timestamp = Some(
                                                            *e.taken_timestamp.as_ref().unwrap(),
                                                        );
                                                        break;
                                                    }
                                                }
                                                if let Some(timestamp_value) = timestamp {
                                                    process_media_tem(
                                                        photo_uri,
                                                        src_path,
                                                        timestamp_value,
                                                    );
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    if media.has_creation_timestamp() {
                                        process_media_tem(
                                            photo_uri,
                                            src_path,
                                            *media.creation_timestamp.as_ref().unwrap(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Completed Posts.");
    Ok(None)
}
