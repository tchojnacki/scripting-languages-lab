use clap::{AppSettings, Parser};

#[derive(Parser)]
#[clap(setting = AppSettings::DeriveDisplayOrder, after_help = "If --column-label is used, first input row is ignored from aggregation.\nGrouping doesn't sort the data - do it yourself.")]
/// Run a given aggregation function on input from stdin and print result to stdout
pub struct AggregateArgs {
    #[clap(
        short,
        long = "aggr",
        require_equals = true,
        possible_values = ["min", "max", "sum", "avg", "count"],
    )]
    /// The aggregation function
    pub aggregation: String,

    #[clap(
        short = 'c',
        long,
        require_equals = true,
        default_value_t = 0,
        conflicts_with = "column-label"
    )]
    /// Index of input column which should be aggregated (conflicts with --column-label)
    pub column_index: usize,

    #[clap(
        short = 'g',
        long,
        require_equals = true,
        use_value_delimiter = true,
        conflicts_with = "column-label"
    )]
    /// Comma separated input column indices to group by before aggregation
    pub group_indices: Vec<usize>,

    #[clap(short = 's', long, require_equals = true, default_value = "\t")]
    /// String used to separate input columns
    pub separator: String,

    #[clap(short = 'C', long, require_equals = true)]
    /// Label of column which should be aggregated (conflicts with --column-index)
    pub column_label: Option<String>,

    #[clap(
        short = 'G',
        long,
        require_equals = true,
        use_value_delimiter = true,
        requires = "column-label"
    )]
    /// Comma separated input column labels to group by before aggregation
    pub group_labels: Vec<String>,
}
