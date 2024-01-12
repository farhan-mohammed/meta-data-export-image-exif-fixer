// src/my_module.rs
use crate::helpers::*;
use serde::Deserialize;
use serde_json::Error as SerdeJsonError;
use std::fs::metadata;

#[derive(Debug, Deserialize)]
struct Json {
    other_photos_v2: Option<Vec<MediaItem>>,
}
impl Json {
    fn has_photos(&self) -> bool {
        return self.other_photos_v2.is_some() && self.other_photos_v2.as_ref().unwrap().len() != 0;
    }
    fn get_photos(&self) -> &Vec<MediaItem> {
        return self.other_photos_v2.as_ref().unwrap();
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
        return self.uri.is_some();
    }
    fn has_timestamp(&self) -> bool {
        return self.creation_timestamp.is_some();
    }
    fn has_metadata(&self) -> bool {
        return self.media_metadata.is_some();
    }

    fn get_uri(&self) -> &String {
        return self.uri.as_ref().unwrap();
    }
    fn get_timestamp(&self) -> &i64 {
        return self.creation_timestamp.as_ref().unwrap();
    }
    fn get_metadata(&self) -> &MediaMetadata {
        return self.media_metadata.as_ref().unwrap();
    }
}

#[derive(Debug, Deserialize)]
struct MediaMetadata {
    photo_metadata: Option<PhotoMetadata>,
}
impl MediaMetadata {
    fn has_photo_metadata(&self) -> bool {
        return self.photo_metadata.is_some();
    }
    fn get_photo_metadata(&self) -> &PhotoMetadata {
        return self.photo_metadata.as_ref().unwrap();
    }
}

#[derive(Debug, Deserialize)]
struct PhotoMetadata {
    exif_data: Option<Vec<ExifData>>,
}
impl PhotoMetadata {
    fn get_exif_data(&self) -> &Vec<ExifData> {
        return self.exif_data.as_ref().unwrap();
    }
    fn has_exif_data(&self) -> bool {
        return self.exif_data.is_some() && self.get_exif_data().len() != 0;
    }
}

#[derive(Debug, Deserialize)]
struct ExifData {
    taken_timestamp: Option<i64>,
}
impl ExifData {
    fn has_taken_timestamp(&self) -> bool {
        return self.taken_timestamp.is_some();
    }
    fn get_taken_timestamp(&self) -> &i64 {
        return self.taken_timestamp.as_ref().unwrap();
    }
}
fn process_media_tem(photo_uri: &str, src_path: &str, last_modified_timestamp: &i64) {
    let uri = remove_prefix(photo_uri, "your_activity_across_facebook/");
    let photo_file_path = &format!("{src_path}/{uri}");
    let timestamp = get_seconds_timestamp(*last_modified_timestamp);
    if metadata(photo_file_path).is_ok() {
        let res = set_photo_taken_time(photo_file_path, timestamp);
        if res.is_ok() {
            // println!("✅ Saved Successfully {photo_file_path} {timestamp}")
        } else {
            // println!("❌ Error Saving file {photo_file_path} {timestamp}");
        }
    }
}

pub fn read_json_and_get_profile_user(src_path: &str) -> Result<Option<String>, SerdeJsonError> {
    let directory_path = format!("posts");
    let jsons = match get_json_file_names(src_path, &directory_path, "your_uncategorized_photos") {
        Ok(messages_json) => messages_json,
        Err(_err) => return Ok(None),
    };
    for json in jsons.iter() {
        println!("{}", json);
        let file = get_file(src_path, &directory_path, &json).expect("file should open read only");
        let photos_json: Json = serde_json::from_reader(file).expect("Unable to read file");
        println!("p1");
      
        if !photos_json.has_photos() {
            break;
        }

        let photos = photos_json.get_photos();
        for photo in photos {
            if photo.has_timestamp() && photo.has_uri() {
                let mut timestamp = photo.get_timestamp();
                println!("{timestamp}");
                if photo.has_metadata() {
                    let metadata = photo.get_metadata();
                    if metadata.has_photo_metadata() {
                        if metadata.get_photo_metadata().has_exif_data() {
                            let exif_data = metadata.get_photo_metadata().get_exif_data();
                            for exif in exif_data {
                                if exif.has_taken_timestamp() {
                                    timestamp = exif.get_taken_timestamp();
                                }
                            }
                        }
                    }
                }
                process_media_tem(&photo.get_uri(), src_path, timestamp)
            }
            // photo.
        }
    }

    println!("Completed Posts.");
    Ok(None)
}
