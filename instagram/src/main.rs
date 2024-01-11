use std::collections::HashSet;
use std::env;
use std::fs;

use std::path::PathBuf;
mod personal_information;
mod archived_posts;
mod stories;
mod messages;
mod posts;
mod helpers;
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
        Ok(entries) => {
            // Collect the names of the subfolders
            let subfolders: HashSet<String> = [
                "ads_information",
                "logged_information",
                "preferences",
                "your_instagram_activity",
                "apps_and_websites_off_of_instagram",
                "media",
                "security_and_login_information",
                "connections",
                "personal_information",
                "your_activity_across_facebook",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect();

            let missing_subfolders: HashSet<String> = subfolders
                .difference(
                    &entries
                        .filter_map(|entry| {
                            entry.ok().and_then(|e| {
                                let path: PathBuf = e.path();
                                let folder_name = path
                                    .file_name()
                                    .and_then(|os_str| os_str.to_str())
                                    .map(|s| s.to_string()); // Clone the string
                                folder_name
                            })
                        })
                        .collect::<HashSet<_>>(),
                )
                .cloned()
                .collect();

            if missing_subfolders.is_empty() {
                println!("All subfolders are present.");
            } else {
                println!("Missing subfolders: {:?}", missing_subfolders);
            }

            let _ = personal_information::read_json_and_get_profile_user(folder_path);
            let _ = archived_posts::read_json_and_get_profile_user(folder_path);

            let _ = stories::read_json_and_get_profile_user(folder_path);
            let _ = posts::read_json_and_get_profile_user(folder_path);
            let _ = messages::read_json_and_get_profile_user(folder_path);

        }
        Err(err) => {
            eprintln!("Error reading folder: {}", err);
            std::process::exit(1);
        }
    }
}
