use crate::helpers::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Json {
    videos_v2: Option<Vec<MediaItem>>,
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
    fn has_timestamp(&self) -> bool {
        self.creation_timestamp.is_some()
    }
    fn get_uri(&self) -> &String {
        self.uri.as_ref().unwrap()
    }
    fn get_timestamp(&self) -> &i64 {
        self.creation_timestamp.as_ref().unwrap()
    }
}

#[derive(Debug, Deserialize)]
struct MediaMetadata {
    video_metadata: Option<PhotoMetadata>,
}
impl MediaMetadata {
    fn get_photo_metadata(&self) -> &PhotoMetadata {
        self.video_metadata.as_ref().unwrap()
    }
}

#[derive(Debug, Deserialize)]
struct PhotoMetadata {
    exif_data: Option<Vec<ExifData>>,
}
#[derive(Debug, Deserialize)]
struct ExifData {
    taken_timestamp: Option<i64>,
}

fn process_media_tem(photo_uri: &str, src_path: &str, last_modified_timestamp: &i64) {
    let uri = remove_prefix(photo_uri, "your_activity_across_facebook/");
    let photo_file_path = &format!("{src_path}/{uri}");
    let timestamp = get_seconds_timestamp(*last_modified_timestamp);
    match set_photo_taken_time(photo_file_path, timestamp) {
        Ok(_) => 1,
        Err(e) => {
            println!("Error setting time for {photo_file_path} {timestamp} - {e}");
            0
        }
    };
}

pub fn execute(src_path: &str) {
    println!("Starting processing post videos");
    let directory_path = "posts";
    match get_json_file_names(src_path, directory_path, "your_videos") {
        Ok(messages_json) => {
            if messages_json.is_empty() {
                println!("No post found related to post videos");
                return;
            }
            for json in messages_json.iter() {
                match get_file(src_path, directory_path, json) {
                    Ok(file) => {
                        let photos_json: Json =
                            serde_json::from_reader::<_, Json>(file).expect("Unable to read file");
                        if let Some(photos) = photos_json.videos_v2 {
                            for photo in photos {
                                if photo.has_timestamp() && photo.has_uri() {
                                    let mut timestamp = *photo.get_timestamp();
                                    if let Some(m) = &photo.media_metadata {
                                        if let Some(exif_data) = &m.get_photo_metadata().exif_data {
                                            for exif in exif_data {
                                                if let Some(ts) = exif.taken_timestamp {
                                                    timestamp = ts;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    process_media_tem(photo.get_uri(), src_path, &timestamp)
                                }
                            }
                        }
                    }
                    Err(err) => {
                        println!("[Error reading {}/{} {err}]", directory_path, json);
                    }
                };
            }
        }
        Err(err) => {
            println!("Error finding jsons: {err}")
        }
    };

    println!("Completed Video Posts.");
}
