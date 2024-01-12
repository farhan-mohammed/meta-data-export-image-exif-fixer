use std::path::Path;
// helpers.rs
use chrono::*;
use std::io::Error;
use std::process::{Command, Output};
use std::{fs::File, io::ErrorKind};

pub fn get_file(src_path: &str, file_path: &str, file: &str) -> Result<File, Error> {
    let mut modified_string = file_path;
    if file_path.ends_with('/') {
        modified_string = file_path.trim_end_matches('/');
    }
    let json_path = format!("{}/{}/{}", src_path, modified_string, file);

    // println!("{json_path}")
    if Path::new(&json_path).exists() {
        File::open(json_path)
    } else {
        Err(Error::new(ErrorKind::NotFound, "File not found"))
    }
}

pub fn get_seconds_timestamp(time: i64) -> i64 {
    // if its s return ms
    // if its ms add EPOCH and return as s
    if time < 1_000_000_000_000 {
        // Assume time is in seconds, convert to milliseconds
        time
    } else {
        // Assume time is in milliseconds, convert to seconds
        // let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        (time) / 1000
    }
}
pub fn remove_prefix(input: &str, prefix: &str) -> String {
    if let Some(suffix) = input.strip_prefix(prefix) {
        suffix.to_string()
    } else {
        input.to_string()
    }
}
pub fn set_photo_taken_time(photo_file_path: &String, timestamp: i64) -> Result<Output, Error> {
    // let photo_path = std::path::Path::new(&photo_file_path);
    // let time = FileTime::from_unix_time(timestamp, 0);

    let dt: DateTime<Utc> = Utc.timestamp_opt(timestamp, 0).unwrap();
    let exif_date_time = dt.format("%Y%m%d%H%M.%S").to_string();
    // println!("{photo_file_path} {exif_date_time}");
    // if  Path::new(&photo_file_path).exists() {
    //     println!("file exists!");
    // } else {
    //     println!("file doesnt exists!");

    // }
    // let command = format!("touch -t {} {}",exif_date_time,photo_file_path);
    // println!("{command}");
    Command::new("touch")
        .arg("-t")
        .arg(exif_date_time)
        .arg(photo_file_path)
        .output()
}

pub fn get_directories(
    src_path: &str,
    directory_path: &str,
) -> Result<Vec<String>, std::io::Error> {
    let full_path: String = format!("{}/{}", src_path, directory_path);
    let entries = match std::fs::read_dir(full_path) {
        Ok(entries) => entries,
        Err(err) => return Err(err),
    };

    let directory_names: Vec<String> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_dir() {
                    Some(path.file_name()?.to_string_lossy().into_owned())
                } else {
                    None
                }
            })
        })
        .collect();

    Ok(directory_names)
    // return all directories inside fullpath
}

pub fn get_json_file_names(
    src_path: &str,
    directory_path: &str,
    prefix_match: &str,
) -> Result<Vec<String>, std::io::Error> {
    let full_path: String = format!("{}/{}", src_path, directory_path);
    let entries = std::fs::read_dir(full_path)?;

    let posts_json_file_names: Vec<String> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file()
                    && path.extension().map_or(false, |ext| ext == "json")
                    && path.file_stem().map_or(false, |stem| {
                        stem.to_string_lossy().starts_with(prefix_match)
                    })
                {
                    Some(path.file_name().unwrap().to_string_lossy().into_owned())
                } else {
                    None
                }
            })
        })
        .collect();

    Ok(posts_json_file_names)
}
