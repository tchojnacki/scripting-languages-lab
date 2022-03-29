use clap::{AppSettings, ArgEnum, Parser};

#[derive(PartialEq, Clone, ArgEnum)]
#[clap(rename_all = lower)]
pub enum FileSorting {
    Alpha,
    Date,
}

#[derive(Parser)]
#[clap(setting = AppSettings::DeriveDisplayOrder, mut_arg("help", |h| h.hide(true)))]
/// Display paths from current directory line by line
pub struct Config {
    #[clap(short = 'R')]
    /// Recursively display nested paths
    pub recursively: bool,

    #[clap(short = 'd')]
    /// Display only directories
    pub directories_only: bool,

    #[clap(short = 's')]
    /// Display file size in bytes after its name
    pub size_displayed: bool,

    #[clap(long = "sort", arg_enum)]
    /// Sort files within directory
    pub sorting: Option<FileSorting>,

    #[clap(
        long = "indent",
        requires = "recursively",
        default_value = "  ",
        hide = true
    )]
    /// Change symbol indicating directory nesting
    pub indent: String,
}
