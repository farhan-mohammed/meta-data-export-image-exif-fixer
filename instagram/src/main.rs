use std::env;

mod archived_posts;
mod helpers;
mod messages;
mod personal_information;
mod posts;
mod stories;

fn main() {
    // Retrieve command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument (the program name) is provided
    if args.len() < 2 {
        println!("Usage: {} <folder_path>", args[0]);
        std::process::exit(1);
    }

    let folder_path = &args[1];
    println!("===== Messages =====");
    messages::execute(folder_path);
    println!("===== Posts =====");
    archived_posts::execute(folder_path);
    posts::execute(folder_path);
    println!("===== Personal Information =====");
    personal_information::execute(folder_path);
    println!("===== Stories =====");
    stories::execute(folder_path);
}
