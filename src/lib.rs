use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
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

pub fn get_dirs_recursively(path: &str, follow_links: bool, include_dot_files: bool) -> Vec<String> {
    let mut files: Vec<String> = vec![];

    for dir in WalkDir::new(&path)
        .follow_links(follow_links)
        .into_iter()
        .filter_map(|dir| dir.ok())
    {
        // the unwrap is find here since we already checked the file (tested in depth)
        if dir.metadata().unwrap().is_dir() {
            if !include_dot_files {
                if dir.file_name().to_str().unwrap().chars().next().unwrap().to_string() == "." {
                    continue;
                }
            }
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

pub fn get_files_in_dir(path: &str, include_dot_files: bool) -> Vec<path::PathBuf> {
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
            if !include_dot_files {
                if &entry
                    .file_name()
                    .to_str()
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap()
                    .to_string()
                    == "."
                {
                    continue;
                }
            }
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
        let extension = match file.extension() {
            Some(ext_os) => match ext_os.to_str() {
                Some(ext) => ext,
                None => {
                    eprintln!("[003] There was an error getting the extension of a file, this may be bug, but please check your file extensions");
                    process::exit(3);
                }
            },
            None => "else",
        };
        extensions.push(extension.to_string().to_uppercase());
    }

    // dedup orig vector
    // sort vector
    extensions.sort();
    // dedup vecotr
    extensions.dedup();

    extensions
}

pub fn get_file_ext_names(files: &Vec<path::PathBuf>) -> HashMap<String, String> {
    let extensions = get_file_extensions(files);

    // Check if the files aren't already sorted aka some extension across the whole folder
    let ext_len = &extensions.len().to_string();
    if ext_len == &"1" || ext_len == &"0" {
        if &files.len().to_string() != "0" {
            let parent = match files[0].parent() {
                Some(root) => root.to_str().unwrap(),
                None => "ROOT",
            };
            println!("ALREADY SORTED DIR: {}", parent);
        }
        return HashMap::new();
    }

    // copy of extensions dupped vec into iter
    let mut extensions_dupped = Vec::new();
    for file in files {
        // the unwrap is fine here since we already validated the metadata
        let extension = match file.extension() {
            Some(ext_os) => match ext_os.to_str() {
                Some(ext) => ext,
                None => {
                    eprintln!("[003] There was an error getting the extension of a file, this may be bug, but please check your file extensions");
                    process::exit(3);
                }
            },
            None => "else",
        };
        extensions_dupped.push(extension.to_string().to_uppercase());
    }
    // sort the dupped vec
    extensions_dupped.sort();
    // reverse the dupped extensions
    extensions_dupped.reverse();

    // compare and get how many items are for each file type.
    let mut ext_count = Vec::new();
    let mut last_count = 0;
    for item in &extensions {
        let index = &extensions_dupped.iter().position(|x| x == item).unwrap();
        let count = &extensions_dupped.len() - index - last_count;
        last_count = last_count + count;
        ext_count.push((count, item));
    }
    // sort the ext_count
    ext_count.sort();

    // nubmer the extensions
    let mut start_number = ext_count.len();
    let mut ext_dir_names = HashMap::new();
    for key in &ext_count {
        ext_dir_names.insert(key.1.to_owned(), start_number.to_string() + "-" + key.1);
        start_number = start_number - 1;
    }

    ext_dir_names
}

pub fn mass_make_dirs(path: &str, names: &Vec<&String>) {
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

pub fn sort_files_by_ext(path: &str, include_dot_files: bool) {
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
    let files = get_files_in_dir(&path, include_dot_files);

    // get their extensions
    let extensions = get_file_ext_names(&files);
    // Check if the files aren't already separated!
    let ext_len = &extensions.len().to_string();
    if ext_len == &"1" || ext_len == &"0" {
        return;
    }

    let folders = extensions.values().collect();

    // create the new dirs
    mass_make_dirs(&path, &folders);

    // move files into the new dir
    for file in files {
        let file_name = match file.file_name() {
            Some(name) => name,
            None => {
                eprintln!("[006] There was an issue getting a file's name, files that terminate in \"..\" are not supported!");
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
        let extension = match &file.extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => "else",
        };
        let destination = root_path.to_owned()
            + &os_sep
            + &extensions[&extension.to_uppercase()]
            + &os_sep
            + file_name.to_str().unwrap();

        // copy the file to the destination
        match fs::copy(&file, &destination) {
            Ok(_) => {
                println!("COPY: {} -> {}", &file.display().to_string(), &destination);
            }
            Err(error) => {
                eprintln!("[008] There was an error copying a file to the newly created dir for it, the given error is: {}", error);
                process::exit(8);
            }
        };

        // now that the copy was successful we can delete the file from its previous location
        match fs::remove_file(&file) {
            Ok(_) => {
                println!("DELETE: {}", &file.display().to_string());
            }
            Err(error) => {
                eprintln!("There was a problem deleting a file that was successfully copied to it's new location, the given error is: {}", error);
            }
        };
    }
}
