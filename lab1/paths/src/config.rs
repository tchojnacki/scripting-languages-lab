enum FileSorting {
    NoSorting,
    Alphabetically,
    ByDate,
}

pub enum ArgumentParsingError<'a> {
    UnknownArgument(&'a str),
    NoSortingType,
    BadSortingType(&'a str),
}

pub struct Config {
    recursively: bool,
    directories_only: bool,
    display_size: bool,
    sorting: FileSorting,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            recursively: false,
            directories_only: false,
            display_size: false,
            sorting: FileSorting::NoSorting,
        }
    }
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Self, ArgumentParsingError> {
        let mut iter = args.iter();
        let mut config = Config::default();

        while let Some(arg) = iter.next() {
            use ArgumentParsingError::*;
            use FileSorting::*;

            match arg.as_str() {
                "-R" => config.recursively = true,
                "-d" => config.directories_only = true,
                "-s" => config.display_size = true,
                "--sort" => {
                    config.sorting = match iter.next() {
                        Some(sorting_type) => match sorting_type.as_str() {
                            "alpha" => Alphabetically,
                            "date" => ByDate,
                            _ => return Err(BadSortingType(sorting_type)),
                        },
                        None => return Err(NoSortingType),
                    }
                }
                _ => return Err(UnknownArgument(arg)),
            }
        }

        Ok(config)
    }
}
