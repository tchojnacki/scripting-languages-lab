use clap::{AppSettings, Parser};

#[derive(Parser, Debug)]
#[clap(setting = AppSettings::DeriveDisplayOrder, mut_arg("help", |h| h.hide(true)))]
/// Parses stdin line by line using rules set by command arguments
pub struct ProcessArgs {
    #[clap(long, require_equals = true, value_name = "n")]
    /// Skip first n characters in line
    ignorefirst: Option<usize>,

    #[clap(long, require_equals = true, value_name = "n")]
    /// Skip last n characters in line
    ignorelast: Option<usize>,

    #[clap(long, require_equals = true, value_name = "str")]
    /// Replace all str occurrences with '\t'
    delimiter: Option<String>,

    #[clap(
        long,
        require_equals = true,
        value_name = "sep",
        requires = "delimiter",
        default_value = "\t"
    )]
    /// Use custom replacement string for --delimiter
    separator: String,

    #[clap(
        long,
        require_equals = true,
        use_value_delimiter = true,
        value_name = "k,l,...,m"
    )]
    /// Display only columns k,l,...,m (after separation)
    project: Vec<usize>,

    #[clap(long, require_equals = true, value_name = "str")]
    /// Display only lines containing str
    select: Option<String>,
}
