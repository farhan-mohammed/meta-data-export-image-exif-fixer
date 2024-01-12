use crate::helpers::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CheckInPosts {
    attachments: Option<Vec<CheckInMedia>>,
}

#[derive(Debug, Deserialize)]
struct CheckInMedia {
    data: Option<Vec<Media>>,
}
#[derive(Debug, Deserialize)]
struct Media {
    media: Option<MediaItem>,
}

#[derive(Debug, Deserialize)]
struct MediaItem {
    uri: Option<String>,
    creation_timestamp: Option<i64>,
    media_metadata: Option<MediaMetadata>,
}

#[derive(Debug, Deserialize)]
struct MediaMetadata {
    photo_metadata: Option<PhotoMetadata>,
}
#[derive(Debug, Deserialize)]
struct PhotoMetadata {
    exif_data: Option<Vec<ExifData>>,
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
    let timestamp = get_seconds_timestamp(last_modified_timestamp);
    match set_photo_taken_time(photo_file_path, timestamp) {
        Ok(_) => 1,
        Err(e) => {
            println!("Error setting time for {photo_file_path} {timestamp} - {e}");
            0
        }
    };
}

pub fn execute(src_path: &str) {
    println!("Starting processing post checkin");

    let directory_path = "posts";
    let jsons = match get_json_file_names(src_path, directory_path, "your_posts__check_ins") {
        Ok(messages_json) => messages_json,
        Err(_err) => {
            print!("Error getting files in directory: {directory_path} {_err}");
            [].to_vec()
        }
    };
    for json in jsons.iter() {
        match get_file(src_path, directory_path, json) {
            Ok(file) => {
                let json: Vec<CheckInPosts> =
                    serde_json::from_reader(file).expect("Unable to read file");
                for media in json {
                    if let Some(attachments) = media.attachments {
                        for checkin_media in attachments.iter() {
                            if let Some(data) = &checkin_media.data {
                                for d in data.iter() {
                                    if let Some(media) = &d.media {
                                        if media.uri.is_none() {
                                            continue;
                                        }
                                        let photo_uri = media.uri.as_ref().unwrap();
                                        if let Some(metadata) =
                                            &media.media_metadata.as_ref().unwrap().photo_metadata
                                        {
                                            if let Some(exif_data) = &metadata.exif_data {
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
                                        if let Some(creation_timestamp) = media.creation_timestamp {
                                            process_media_tem(
                                                photo_uri,
                                                src_path,
                                                creation_timestamp,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => {
                println!("[Error reading {}/{} {err}]", directory_path, json);
            }
        };
    }
    println!("Completed checkin Posts.");
}
