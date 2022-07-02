use std::env;
use std::fs;
use std::path;
use std::process;
use walkdir::WalkDir;

pub fn get_os_sep() -> String {
    if cfg!(windows) {
        return String::from("\\");
    } else if cfg!(unix) {
        return String::from("/");
    } else {
        eprintln!("Your OS is not supported!");
        process::exit(201);
    }
}

pub fn get_dirs_recursively(path: &str) -> Vec<String> {
    let mut files: Vec<String> = vec![];

    for dir in WalkDir::new(&path)
        .follow_links(true)
        .into_iter()
        .filter_map(|dir| dir.ok())
    {
        // the unwrap is find here since we already checked the file (tested in depth)
        if dir.metadata().unwrap().is_dir() {
            files.push(dir.path().display().to_string());
        }
    }

    files
}

pub fn get_current_dir() -> String {
    match env::current_dir() {
        Ok(dir) => dir.display().to_string(),
        Err(error) => {
            eprintln!(
                "[001] There was an error getting the current dir path, the given error is: {}",
                error
            );
            process::exit(1);
        }
    }
}

pub fn get_files_in_dir(path: &str) -> Vec<path::PathBuf> {
    let read = match fs::read_dir(&path) {
        Ok(read) => read,
        Err(error) => {
            eprintln!(
                "[002] There was an error reading a directory, the given error is: {}",
                error
            );
            process::exit(2);
        }
    };

    let mut files = Vec::new();
    for entry in read.filter_map(|file| file.ok()) {
        // the unwrap is find here since we already checked the file (tested in depth)
        if entry.metadata().unwrap().is_file() {
            files.push(entry.path());
        }
    }

    files
}

pub fn get_file_extensions(files: &Vec<path::PathBuf>) -> Vec<String> {
    // let files = get_files_in_dir(&path);
    let mut extensions = Vec::new();
    for file in files {
        // the unwrap is fine here since we already validated the metadata
        let extension = match file.extension().unwrap().to_str() {
            Some(ext) => ext,
            None => {
                eprintln!("[003] There was an error getting the extension of a file, this may be bug, but please check your file extensions");
                process::exit(3);
            }
        };
        extensions.push(extension.to_string().to_uppercase());
    }
    // sort vector
    extensions.sort();
    // dedup vecotr
    extensions.dedup();

    extensions
}

pub fn mass_make_dirs(path: &str, names: &Vec<String>) {
    match env::set_current_dir(&path) {
        Ok(_) => (),
        Err(error) => {
            eprintln!(
                "[004] There was an error changing the current dir, the given error is: {}",
                error
            );
            process::exit(4);
        }
    };

    for name in names {
        match fs::create_dir(name) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("[005] There was an error creating a new dir inside the dir/one of the sub dirs, the given error is: {}", error);
                process::exit(5);
            }
        };
    }
}

pub fn sort_files_by_ext(path: &str) {
    // get os separator
    let os_sep = get_os_sep();

    // change currrent dir to given path
    match env::set_current_dir(&path) {
        Ok(_) => (),
        Err(error) => {
            eprintln!(
                "[004] There was an error changing the current dir, the given error is: {}",
                error
            );
            process::exit(4);
        }
    };

    // get the files in dir
    let files = get_files_in_dir(&path);

    // get their extensions
    let extensions = get_file_extensions(&files);

    // create the new dirs
    mass_make_dirs(&path, &extensions);

    // move files into the new dir
    for file in files {
        let file_name = match file.file_name() {
            Some(name) => name,
            None => {
                eprintln!("[006] There was an issue getting a files name, files that terminate in \"..\" are not supported!");
                process::exit(6);
            }
        };

        let root_path = match file.parent() {
            Some(path) => match path.to_str() {
                Some(str) => str,
                None => {
                    eprintln!("[007] There was a problem converting a path to string!");
                    process::exit(7);
                }
            },
            None => {
                eprintln!("[008] There was a problem getting the parent directory of a path!");
                process::exit(8);
            }
        };
        // the first unwrap is find because we already validated the metadata, the second one is also fine because it would have already stopped from the operations before if it wasn't valid, the third unwrap is also valid since the whole path string got validated.
        let destination = root_path.to_owned()
            + &os_sep
            + &file.extension().unwrap().to_str().unwrap().to_uppercase()
            + &os_sep
            + file_name.to_str().unwrap();

        // copy the file to the destination
        match fs::copy(&file, destination) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("[008] There was an error copying a file to the newly created dir for it, the given error is: {}", error);
                process::exit(8);
            }
        };

        // now that the copy was successful we can delete the file from its previous location
        match fs::remove_file(file) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("There was a problem deleting a file that was successfully copied to it's new location, the given error is: {}", error);
            }
        };
    }
}
