use crate::helpers::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Json {
    other_photos_v2: Option<Vec<MediaItem>>,
}
impl Json {
    fn get_photos(&self) -> &Vec<MediaItem> {
        self.other_photos_v2.as_ref().unwrap()
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
    photo_metadata: Option<PhotoMetadata>,
}
impl MediaMetadata {
    fn get_photo_metadata(&self) -> &PhotoMetadata {
        self.photo_metadata.as_ref().unwrap()
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
    println!("Starting processing uncategoried posts");
    let directory_path = "posts";
    match get_json_file_names(src_path, directory_path, "your_uncategorized_photos") {
        Ok(jsons) => {
            if jsons.is_empty() {
                println!("No jsons found related to uncategorized posts");
                return;
            }
            for json in jsons.iter() {
                match get_file(src_path, directory_path, json) {
                    Ok(file) => {
                        let photos_json: Json =
                            serde_json::from_reader(file).expect("Unable to read file");
                        for photo in photos_json.get_photos() {
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
                    Err(err) => {
                        println!("[Error reading {directory_path}/{json} {err}]");
                    }
                };
            }
        }
        Err(err) => {
            println!("Error finding jsons: {err}")
        }
    };

    println!("Completed Uncategorized Posts.");
}
