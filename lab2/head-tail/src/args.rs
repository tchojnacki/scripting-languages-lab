use clap::Parser;

pub trait CommonArgs {
    fn expected_lines(&self) -> usize;
    fn error_suppression(&self) -> bool;
}

#[derive(Parser)]
#[clap(name = "myhead", mut_arg("help", |h| h.hide(true)))]
/// Alternative to head(1) from Linux
pub struct HeadArgs {
    /// Print the first n lines
    #[clap(long = "lines", require_equals(true), value_name("n"))]
    expected_lines: usize,

    /// Don't print error when some lines are missing
    #[clap(short)]
    error_suppression: bool,
}

impl CommonArgs for HeadArgs {
    fn expected_lines(&self) -> usize {
        self.expected_lines
    }

    fn error_suppression(&self) -> bool {
        self.error_suppression
    }
}

#[derive(Parser)]
#[clap(name = "mytail", mut_arg("help", |h| h.hide(true)))]
/// Alternative to tail(1) from Linux
pub struct TailArgs {
    /// Print the last n lines
    #[clap(long = "lines", require_equals(true), value_name("n"))]
    expected_lines: usize,

    /// Don't print error when some lines are missing
    #[clap(short)]
    error_suppression: bool,
}

impl CommonArgs for TailArgs {
    fn expected_lines(&self) -> usize {
        self.expected_lines
    }

    fn error_suppression(&self) -> bool {
        self.error_suppression
    }
}
