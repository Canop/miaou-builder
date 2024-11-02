use {
    crate::*,
    clap::{
        CommandFactory,
        Parser,
    },
};

static INTRO: &str = "

a builder for miaou
";

/// Launch arguments
#[derive(Debug, Parser)]
#[command(
    author,
    about,
    version,
    disable_version_flag = true,
    disable_help_flag = true
)]
pub struct Args {
    /// Print help information
    #[arg(long)]
    pub help: bool,

    /// Print the version
    #[arg(long)]
    pub version: bool,
}
