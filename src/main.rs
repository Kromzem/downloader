use std::{
    env,
    fs::{self, File},
    path::{Path, PathBuf},
};

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let output_dir_path: PathBuf = dotenv::var("OUTPUT_DIR")
        .expect("No output dir defined!")
        .into();
    let queue_file_path: PathBuf = dotenv::var("QUEUE_FILE")
        .expect("No queue file path defined!")
        .into();

    let queue = fs::read_to_string(queue_file_path).expect("Failed to read queue file");

    for (file_name, url) in queue.lines().map(parse_queue_line) {
        let file_path = Path::new(&output_dir_path).join(file_name);

        if file_path.exists() {
            println!("Skipping download of '{}' ...", file_name);
            continue;
        }

        println!("Starting download of '{}' ...", file_name);

        let response = match ureq::get(url).call() {
            Ok(response) => response,
            Err(e) => {
                println!("Request failure! {:?}", e);
                continue;
            }
        };

        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(e) => {
                println!("File failure! {:?}", e);
                continue;
            }
        };

        match std::io::copy(&mut response.into_reader(), &mut file) {
            Ok(_) => {
                println!("Success!");
            }
            Err(e) => {
                println!("Download failure! {:?}", e)
            }
        }
    }

    println!("Finished!");
}

fn parse_queue_line(line: &str) -> (&str, &str) {
    line.split_once(';')
        .expect(&format!("Line '{}' not parsable", line))
}
