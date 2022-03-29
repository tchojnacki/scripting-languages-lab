use std::{env, error::Error, fs, path::Path, time::SystemTime};

use crate::{config::FileSorting, Config};

pub fn traverse(config: Config) -> Result<(), Box<dyn Error>> {
    recurse_directory(&env::current_dir()?, &config, 0)
}

fn recurse_directory(path: &Path, config: &Config, depth: usize) -> Result<(), Box<dyn Error>> {
    let mut subpaths = fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    match config.sorting {
        Some(FileSorting::Alpha) => subpaths.sort_by_key(|p| get_file_name(p).to_owned()),
        Some(FileSorting::Date) => subpaths.sort_by_key(|p| get_modification_time(p).to_owned()),
        None => (),
    };

    for path in subpaths {
        let file_name = get_file_name(&path);

        if !config.directories_only || path.is_dir() {
            let padding = config.indent.repeat(depth);

            print!("{padding}{file_name}");

            if config.size_displayed && !path.is_dir() {
                let file_size = fs::metadata(&path)?.len();
                println!("\t{file_size}");
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
