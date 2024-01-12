use std::env;
use std::fs;

mod albums;
mod archived;
mod checkinposts;
mod helpers;
mod uncategorizedphotos;
mod videos;
fn main() {
    // Retrieve command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument (the program name) is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <folder_path>", args[0]);
        std::process::exit(1);
    }

    // Extract the folder path from the command line arguments
    let folder_path = &args[1];

    // List the contents of the folder
    match fs::read_dir(folder_path) {
        Ok(_entries) => {
            // let _ = personal_information::read_json_and_get_profile_user(folder_path);
            // let _ = archived_posts::read_json_and_get_profile_user(folder_path);
            // let _ = stories::read_json_and_get_profile_user(folder_path);
            // let _ = posts::read_json_and_get_profile_user(folder_path);
            // inbox / Archived, messages e2eee.json
            let _ = archived::read_json_and_get_profile_user(folder_path);

            // Posts
            let _ = albums::read_json_and_get_profile_user(folder_path);
            let _ = checkinposts::read_json_and_get_profile_user(folder_path);
            let _ = videos::read_json_and_get_profile_user(folder_path);
            let _ = uncategorizedphotos::read_json_and_get_profile_user(folder_path);

            // Posts
        }
        Err(err) => {
            eprintln!("Error reading folder: {}", err);
            std::process::exit(1);
        }
    }
}
