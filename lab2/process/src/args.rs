use clap::{AppSettings, Parser};

const INVALID_USAGE_ERROR_CODE: i32 = 64;

#[derive(Parser, Debug)]
#[clap(setting = AppSettings::DeriveDisplayOrder, mut_arg("help", |h| h.hide(true)))]
/// Parses stdin line by line using rules set by command arguments
pub struct ProcessArgs {
    #[clap(long, require_equals = true, value_name = "n", default_value_t = 0)]
    /// Skip first n characters in each line
    pub ignorefirst: usize,

    #[clap(long, require_equals = true, value_name = "n", default_value_t = 0)]
    /// Skip last n characters in each line
    pub ignorelast: usize,

    #[clap(long, require_equals = true, value_name = "str", default_value = ",")]
    /// Replace all str occurrences with sep
    pub delimiter: String,

    #[clap(long, require_equals = true, value_name = "sep", default_value = "\t")]
    /// Use custom replacement string for --delimiter
    pub separator: String,

    #[clap(
        long,
        require_equals = true,
        use_value_delimiter = true,
        value_name = "k,l,...,m"
    )]
    /// Display only columns k,l,...,m (after separation)
    pub project: Option<Vec<usize>>,

    #[clap(long, require_equals = true, value_name = "str")]
    /// Display only lines containing str
    pub select: Option<String>,
}

pub fn parse_args_or_exit() -> ProcessArgs {
    match ProcessArgs::try_parse() {
        Ok(args) => args,
        Err(e) => {
            let _ = e.print();
            std::process::exit(INVALID_USAGE_ERROR_CODE)
        }
    }
}
