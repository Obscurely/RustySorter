use rusty_sorter;
use std::env;
use std::time::Instant;

fn main() {
    // TEMP: change to work dir
    env::set_current_dir("/home/netrunner/Code/Rust/rusty-sorter/work/").unwrap();

    // Settings
    let mut recursive = false;
    let mut follow_links = false;
    let mut include_dot_files = false;
    
    // Cli options
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("An open source, fast, simple yet efficient file sorter with recursive capabilities!");
        ap.refer(&mut recursive).add_option(
            &["-r", "--recursive"],
            argparse::StoreTrue,
            "Recursive sorting, goes into all sub dirs.",
        );
        ap.refer(&mut follow_links).add_option(
            &["-f", "--follow-links"],
            argparse::StoreTrue,
            "Follow links, sym-links, shortcuts, etc.",
        );
        ap.refer(&mut include_dot_files).add_option(
            &["-d", "--include-dot-files"],
            argparse::StoreTrue,
            "Sorts by including dot files and dot dirs.",
        );
        ap.parse_args_or_exit();
    }

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
