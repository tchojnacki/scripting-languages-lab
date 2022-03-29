use std::{env, fs, path::Path, time::SystemTime};

use crate::{config::FileSorting, Config};

pub fn traverse(config: &Config) {
    recurse_directory(
        &env::current_dir().expect("Can not obtain current dir."),
        config,
        0,
    )
}

fn recurse_directory(path: &Path, config: &Config, depth: usize) {
    if let Ok(dir) = fs::read_dir(path) {
        let mut subpaths = dir
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect::<Vec<_>>();

        match config.sorting {
            Some(FileSorting::Alpha) => subpaths.sort_by_key(|p| get_file_name(p).to_owned()),
            Some(FileSorting::Date) => {
                subpaths.sort_by_key(|p| get_modification_time(p).to_owned())
            }
            None => (),
        };

        for path in subpaths {
            let file_name = get_file_name(&path);

            if !config.directories_only || path.is_dir() {
                let padding = config.indent.repeat(depth);

                print!("{padding}{file_name}");

                if config.size_displayed && !path.is_dir() {
                    let file_size = get_file_size(&path);
                    println!("\t{file_size}");
                } else {
                    println!();
                }
            }

            if config.recursively && path.is_dir() {
                recurse_directory(&path, config, depth + 1);
            }
        }
    }
}

fn get_file_name(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
}

fn get_file_size(path: &Path) -> u64 {
    fs::metadata(&path).map(|data| data.len()).unwrap_or(0)
}

fn get_modification_time(path: &Path) -> SystemTime {
    fs::metadata(path)
        .and_then(|data| data.modified())
        .unwrap_or_else(|_| SystemTime::now())
}
