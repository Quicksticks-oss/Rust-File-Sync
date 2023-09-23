use walkdir::WalkDir;
use std::fs;

pub fn default_callback(_:String, _:String, file_name_str:String){
    println!("Scanning: {}", file_name_str);
}

pub fn create_dir(path:String){
    // Attempt to create the directory
    match fs::create_dir(&path) {
        Ok(_) => {},
        Err(_) => {},
    }
}

pub fn delete_dir(path:String){
    // Attempt to remove the directory
    match fs::remove_dir_all(&path) {
        Ok(_) => {},
        Err(_) => {},
    }
}

fn compare(source_path:&String, dest_path:&String, source: Vec<(String, String)>, dest: Vec<(String, String)>) -> (Vec<String>, Vec<String>){
    // Find files to modify, create, or destroy
    let mut creations = vec![];
    let mut destructions = vec![];

    // Compare source to dest
    for (source_name, source_content) in &source {
        let source_name_new = &source_name.replace(source_path, dest_path);
        if let Some(_) = dest.iter().position(|(name, _)| name == source_name_new) {
            if dest.contains(&(source_name_new.clone(), source_content.clone())) == false {
                // Modify file.
                creations.push(source_name.clone());
            }
        } else {
            // Create new file.
            creations.push(source_name.clone());
        }
    }

    // Find files to destroy
    for (dest_name_, _) in &dest {
        let dest_name_str = dest_name_.replace(dest_path, &source_path);
        let dest_name = &dest_name_str;
        if !source.iter().any(|(source_name, _)| source_name == dest_name && source_path != dest_name && dest_path != dest_name) {
            destructions.push(dest_name.clone());
        }
    }

    if creations.len() > 0{
        creations.remove(0);
    }
    if destructions.len() > 0{
        destructions.remove(0);
    }

    return (creations, destructions);
}

pub fn sync(
    source_path: &String,
    dest_path: &String,
    source: (Vec<(String, String)>, Vec<(String, String)>),
    destination: (Vec<(String, String)>, Vec<(String, String)>),
) {
    // Extract source and destination file lists and directories
    let (source_files, source_dirs) = source;
    let (dest_files, destination_dirs) = destination;

    let (dir_creations, dir_destructions) = compare(source_path, dest_path, source_dirs, destination_dirs);
    let (file_creations, file_destructions) = compare(source_path, dest_path, source_files, dest_files);

    // Create new dirs.
    dir_creations.iter().for_each(|item| {
        let new_dir = item.replace(source_path, dest_path);
        create_dir(new_dir.clone());
    });

    // Delete dirs.
    dir_destructions.iter().for_each(|item| {
        let new_dir = item.replace(source_path, dest_path);
        delete_dir(new_dir.clone());
    });

    // Copy Files
    file_creations.iter().for_each(|item| {
        let new_dir = item.replace(source_path, dest_path);
        println!("Copy: {} to {}", item, new_dir);
        match fs::copy(item, new_dir.clone()) {
            Ok(_) => {},
            Err(err) => eprintln!("Error: {}", err),
        }
    });

    file_destructions.iter().for_each(|item| {
        let new_file = item.replace(source_path, dest_path);
        println!("Del: {}", new_file);
        // Attempt to delete the file
        match fs::remove_file(new_file) {
            Ok(_) => println!("File deleted successfully."),
            Err(e) => eprintln!("Error deleting file: {}", e),
        }
    });
}

pub fn walk_folder(source_path:&str, callback: &dyn Fn(String, String, String)) -> (Vec<(String, String)>, Vec<(String, String)>){
    let mut files:Vec<(String, String)> = vec![];
    let mut directories:Vec<(String, String)> = vec![];
    for entry in WalkDir::new(source_path).follow_links(true) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    let file_name_str: String = entry.file_name().to_string_lossy().into_owned();
                    let full_file_path: String = entry.path().to_string_lossy().into_owned();
                    let file_size = entry.metadata().expect("Could not get metadata.").len(); // Get file size in bytes
                    files.push((full_file_path.clone(), file_size.to_string().clone()));
                    callback(full_file_path, file_size.to_string(), file_name_str);
                }
                else if entry.file_type().is_dir() {
                    let file_name_str: String = entry.file_name().to_string_lossy().into_owned();
                    let full_file_path: String = entry.path().to_string_lossy().into_owned();
                    let file_size = entry.metadata().expect("Could not get metadata.").len(); // Get file size in bytes
                    directories.push((full_file_path.clone(), file_name_str.clone()));
                    callback(full_file_path, file_size.to_string(), file_name_str);
                }
            }
            Err(err) => {
                // Handle the error here, e.g., print the error message
                eprintln!("Error: {}", err);
            }
        }
    }
    return (files, directories);
}