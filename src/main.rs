use rusty_sorter;
use std::env;
use std::time::Instant;

fn main() {
    // TEMP: change to work dir
    env::set_current_dir("/home/netrunner/Code/Rust/rusty-sorter/work/").unwrap();

    // Settings
    let recursive = true;
    let follow_links = true;
    let include_dot_files = true;

    // get current dir
    let current_dir = rusty_sorter::get_current_dir();

    // start messuring execution time from now
    let now = Instant::now();

    // main functionality, with check on settings and everything
    if recursive {
        // get dir recursively (include sub dirs) in the current dir
        let dirs = rusty_sorter::get_dirs_recursively(&current_dir, follow_links, include_dot_files);

        for dir in dirs {
            rusty_sorter::sort_files_by_ext(&dir, include_dot_files);
        }
    } else {
        rusty_sorter::sort_files_by_ext(&current_dir, include_dot_files);
    }

    // print execution time
    let elapsed = Instant::now() - now;
    println!("Successfully sorted all given files, Time Elapsed: {}ms", elapsed.as_millis());
}
