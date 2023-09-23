mod sync {
    pub mod syncer;
}
use std::time::Instant;
use std::error::Error;
use std::fs;
use std::env;

use crate::sync::syncer;

fn create_dir_non_exists(dir_path:&str){
    // Check if the directory exists
    if !fs::metadata(&dir_path).is_ok() {
        // If it doesn't exist, create it
        if let Err(err) = fs::create_dir(&dir_path) {
            eprintln!("Error creating directory: {}", err);
        } else {
            println!("Directory created: {}", dir_path);
        }
    } else {
        println!("Directory already exists: {}", dir_path);
    }
}

fn add_trailing_slash(mut input: String) -> String {
    if !input.ends_with('/') {
        input.push('/');
    }
    input
}

fn print_help() {
    println!("Usage: sync <source_path> <destination_path>");
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Made by: https://github.com/Quicksticks-oss");
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_help();
        return Ok(());
    }

    let sync_source = String::from(&args[1]);
    let sync_destination = String::from(&args[2]);

    let sync_source_path = &add_trailing_slash(sync_source);
    let sync_destination_path = &add_trailing_slash(sync_destination);
    

    create_dir_non_exists(sync_destination_path);

    let sync_source_path_string: &String = &String::from(sync_source_path);
    let sync_destination_path_string: &String = &String::from(sync_destination_path);

    println!("Starting Sync...");
    let start_time = Instant::now();

    let source_paths = syncer::walk_folder(sync_source_path, &syncer::default_callback);
    let dest_paths = syncer::walk_folder(sync_destination_path, &syncer::default_callback);

    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("Time taken to scan: {:.2} seconds", duration.as_secs_f64());
    let start_time = Instant::now();

    syncer::sync(sync_source_path_string, sync_destination_path_string, source_paths, dest_paths);

    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("Time taken to sync: {:.2} seconds", duration.as_secs_f64());

    Ok(())
}