# Rusty Sorter
## About project
An open source, fast, simple yet efficient file sorter with recursive capabilities!
## How it works
It takes some command line arguments than sorts your files by putting them in folder named by their extension with a number in front (like 1-png) based on which files are more of the extensions in order.
## Command line arguments
Usage:
  rusty-sorter [OPTIONS]

Optional arguments:
  -h,--help             Show this help message and exit
  -r,--recursive        Recursive sorting, goes into all sub dirs.
  -f,--follow-links     Follow links, sym-links, shortcuts, etc.
  -d,--include-dot-files
                        Sorts by including dot files and dot dirs.
