use crate::helpers::{get_file, get_seconds_timestamp, set_photo_taken_time};

use serde::Deserialize;
use serde_json::Error as SerdeJsonError;
use std::fs::metadata;

#[derive(Debug, Deserialize)] // Derive Deserialize trait for your struct
struct MyConfig {
    profile_user: Option<Vec<ProfileUser>>,
}
impl MyConfig {
    fn has_profile_user(&self) -> bool {
        self.profile_user.is_some() || self.profile_user.is_none()
    }
}

#[derive(Debug, Deserialize)]
struct ProfileUser {
    media_map_data: Option<MediaMapData>,
}
impl ProfileUser {
    fn has_media_data(&self) -> bool {
        self.media_map_data.is_some()
    }
}

#[derive(Debug, Deserialize)]
struct MediaMapData {
    #[serde(rename = "Profile Photo")]
    profile_photo: Option<ProfilePhoto>,
}
impl MediaMapData {
    fn has_profile_photo(&self) -> bool {
        self.profile_photo.is_some()
    }
}

#[derive(Debug, Deserialize)]
struct ProfilePhoto {
    uri: String,
    creation_timestamp: i64,
}

pub fn read_json_and_get_profile_user(src_path: &str) -> Result<Option<String>, SerdeJsonError> {
    let json_name = "personal_information.json";
    let file_path = "personal_information/personal_information";

    let file = get_file(src_path, file_path, json_name).expect("file should open read only");
    let json: MyConfig = serde_json::from_reader(file)
        .unwrap_or_else(|_| panic!("Unable to find/read {}", json_name));

    if !json.has_profile_user() {
        println!("Error Missing Json: {json_name}");
        return Ok(None);
    }

    let profile_user = &json.profile_user.unwrap();
    for element in profile_user.iter() {
        if !element.has_media_data() {
            continue;
        }

        let media_map_data = element.media_map_data.as_ref().unwrap();
        if !media_map_data.has_profile_photo() {
            continue;
        }

        let profile_photo = media_map_data.profile_photo.as_ref().unwrap();
        let photo_file_path = &format!("{}/{}", src_path, profile_photo.uri);

        let timestamp = get_seconds_timestamp(profile_photo.creation_timestamp);

        if metadata(photo_file_path).is_ok() {
            let _ = set_photo_taken_time(photo_file_path, timestamp);
            // if res.is_ok() {
            //     println!("✅ Saved Successfully {photo_file_path} {timestamp}")
            // } else {
            //     println!("❌ Error Saving file {photo_file_path} {timestamp}");
            // }
        }
    }

    println!("Completed Personal Infomration");
    Ok(None)
}
