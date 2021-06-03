use std::ops::Deref;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Command {
    #[structopt(name = "flaky")]
    #[structopt(
        after_help = "This command allows you to run your tests an arbitrary number of times to try
        to find flaky tests, return as report of the found failing tests."
    )]
    Flaky(Args),
}

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
pub struct Args {
    /// Whether to run the tests in release mode.
    #[structopt(long)]
    pub release: bool,
    /// The number of times the tests have to be ran.
    #[structopt(long, short, default_value = "100")]
    pub repeat: usize,
    /// If set, runs for all the iteration defined by repeat, otherwise, stops as soon as a faling
    /// test is found.
    #[structopt(long, short)]
    pub exhaustive: bool,
    /// Pass custom arguments to cargo test.
    #[structopt(long, short)]
    pub args: Option<String>,
}

impl Deref for Command {
    type Target = Args;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Flaky(ref args) => args
        }
    }
}