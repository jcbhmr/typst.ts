use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};
use typst_ts_core::build_info::VERSION;

pub mod http;
pub mod utils;

#[derive(Debug, Parser)]
#[clap(name = "typst-ts-dev-server", version = VERSION)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "The dev-server for typst.ts.",
    after_help = "",
    next_display_order = None
)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    /// Compile Commands
    Compile(CompileOpts),
    /// Run server
    #[clap(subcommand)]
    Run(RunSubCommands),
}

#[derive(Debug, Parser)]
#[clap(
    about = "Commands about compile alias for typst.",
    after_help = "",
    next_display_order = None
)]
#[allow(clippy::large_enum_variant)]
pub struct CompileOpts {
    #[clap(subcommand)]
    pub sub: CompileSubCommands,

    /// Using underlying compiler to compile corpus.
    #[clap(long, default_value = "")]
    pub compiler: String,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "Commands about compile alias for typst.",
    after_help = "",
    next_display_order = None
)]
#[allow(clippy::large_enum_variant)]
pub enum CompileSubCommands {
    /// Compile corpus for typst.ts.
    Corpus(CompileCorpusArgs),
}

#[derive(Debug, Clone, Parser)]
#[clap(next_help_heading = "Compile Corpus options")]
pub struct CompileCorpusArgs {
    /// The name of Corpus.
    #[clap(long = "cat", value_name = "CAT", value_delimiter = ',', action = ArgAction::Append)]
    pub catergories: Vec<String>,

    /// Output formats.
    #[clap(long)]
    pub format: Vec<String>,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "Commands about run for typst.",
    after_help = "",
    next_display_order = None
)]
#[allow(clippy::large_enum_variant)]
pub enum RunSubCommands {
    /// Run HTTP server
    Http(RunHttpArgs),
}

#[derive(Debug, Clone, Parser)]
#[clap(next_help_heading = "Run options")]
pub struct RunHttpArgs {
    /// The corpus directory.
    #[clap(long)]
    pub corpus: PathBuf,

    /// Listen address.
    #[clap(long, default_value = "")]
    pub http: String,
}
