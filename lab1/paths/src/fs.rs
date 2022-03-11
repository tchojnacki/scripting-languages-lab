use std::{env, error::Error, fs, os::windows::prelude::MetadataExt, path::Path, time::SystemTime};

use crate::{config::FileSorting, Config};

const DEPTH_PADDING_MULTIPLIER: usize = 4;

pub fn traverse(config: Config) -> Result<(), Box<dyn Error>> {
    recurse_directory(&env::current_dir()?, &config, 0)
}

fn recurse_directory(path: &Path, config: &Config, depth: usize) -> Result<(), Box<dyn Error>> {
    let mut subpaths = fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    match config.sorting {
        FileSorting::Alphabetically => subpaths.sort_by_key(|p| get_file_name(p).to_owned()),
        FileSorting::ByDate => subpaths.sort_by_key(|p| get_modification_time(p).to_owned()),
        FileSorting::NoSorting => (),
    };

    for path in subpaths {
        let file_name = get_file_name(&path);

        if !config.directories_only || path.is_dir() {
            let padding = " ".repeat(DEPTH_PADDING_MULTIPLIER * depth);

            print!("{padding}{file_name}");

            if config.display_size && !path.is_dir() {
                let file_size = fs::metadata(&path)?.file_size();
                println!("\t\t{file_size}");
            } else {
                println!();
            }
        }

        if config.recursively && path.is_dir() {
            recurse_directory(&path, config, depth + 1)?;
        }
    }

    Ok(())
}

fn get_file_name(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
}

fn get_modification_time(path: &Path) -> SystemTime {
    fs::metadata(path)
        .and_then(|data| data.modified())
        .unwrap_or_else(|_| SystemTime::now())
}
