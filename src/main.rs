use rusty_sorter;
use std::env;
use std::process;
use std::time::Instant;

fn main() {
    // TEMP: change to work dir
    env::set_current_dir("/home/netrunner/Code/Rust/rusty-sorter/work/").unwrap();

    // get current dir
    let current_dir = rusty_sorter::get_current_dir();

    let now = Instant::now();
    // get dir recursively (include sub dirs) in the current dir
    let dirs = rusty_sorter::get_dirs_recursively(&current_dir);

    // println!("{}", dirs.len());
    for dir in dirs {
        rusty_sorter::sort_files_by_ext(&dir);
        // println!("{}", dir);
    }
    let elapsed = Instant::now() - now;
    println!("Elapsed: {}ms", elapsed.as_millis());
}
