pub mod topics;
pub mod push;
pub mod pull;

use clap::{Args, ArgEnum, ArgGroup, AppSettings, ArgSettings, Parser};
use topics::Topics;
use push::Push;
use pull::Pull;

/// Manage Kafka clusters and send + receive messages to + from topics
#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Simon Collins <hello@simoncollins.dev>")]
pub struct Options {
    /// Boostrap brokers (comma separated) list.
    /// Mandatory if not configured via .junctionrc
    #[clap(short, long, setting(ArgSettings::UseValueDelimiter))]
    brokers: Option<Vec<String>>,

    #[clap(subcommand)]
    pub sub_cmd: RootSubCommand,
}

#[derive(Parser, Debug)]
pub enum RootSubCommand {
    #[clap()]
    Topics(Topics),
    Push(Push),
    Pull(Pull)
}

pub fn parse_options() -> Options {
    Options::parse()
}