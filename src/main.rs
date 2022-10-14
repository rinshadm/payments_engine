use std::env;

mod engine;
mod entities;
mod utils;

fn main() {
    let file_name = match env::args().nth(1) {
        Some(file_name) => file_name,
        None => {
            eprintln!("File name argument not found. Exiting!");
            return;
        }
    };

    if !utils::check_file_exists(&file_name) {
        eprintln!(
            "File {} does not exist. Please check the path is correct. Exiting!",
            file_name
        );
        return;
    }

    if let Err(error) = engine::run(&file_name) {
        eprintln!("Engine failed to run with error: {}", error);
        return;
    }
}
