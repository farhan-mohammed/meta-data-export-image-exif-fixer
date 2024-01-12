use std::env;
mod albums;
mod checkinposts;
mod helpers;
mod messages;
mod uncategorizedphotos;
mod videos;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <folder_path>", args[0]);
        std::process::exit(1);
    }
    let folder_path = &args[1];
    // Messages
    println!("===== Messages =====");
    messages::execute(folder_path);
    // Posts
    println!("===== Posts =====");
    albums::execute(folder_path);
    checkinposts::execute(folder_path);
    videos::execute(folder_path);
    uncategorizedphotos::execute(folder_path);
}
