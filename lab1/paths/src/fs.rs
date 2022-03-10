use std::{env, error::Error, path::Path};

use crate::Config;

pub fn traverse(config: Config) -> Result<(), Box<dyn Error>> {
    recurse_directory(&env::current_dir()?);

    Ok(())
}

fn recurse_directory(path: &Path) {}
