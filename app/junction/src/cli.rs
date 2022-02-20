use clap::{Args, ArgEnum, ArgGroup, AppSettings, ArgSettings, Parser};

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

#[derive(Parser, Debug)]
pub enum TopicsSubCommand {
    #[clap()]
    Delete(DeleteTopic),
    List(ListTopics),
    Create(CreateTopic)
}

/// Manage topics
#[derive(Parser, Debug)]
pub struct Topics {
    #[clap(subcommand)]
    pub sub_cmd: TopicsSubCommand,
}

/// Read messages from topics
#[derive(Parser, Debug)]
pub struct Push {
    /// The topics to read from (comma delimited)
    #[clap(short, long = "topics", value_name = "TOPICS", setting(ArgSettings::UseValueDelimiter))]
    topic_names: Vec<String>
}

/// Send messages to a topic
#[derive(Parser, Debug)]
pub struct Pull {
    /// The topics to send data to
    #[clap(short, long = "topics", value_name = "TOPICS", setting(ArgSettings::UseValueDelimiter))]
    topic_names: Vec<String>
}

/// Create a new topic
#[derive(Parser, Debug)]
pub struct CreateTopic {
    /// The topic to create
    topic_name: String
}

/// Send messages to a topic
#[derive(Parser, Debug)]
pub struct DeleteTopic {
    /// The topic to delete
    topic_name: String
}

/// List topics on the cluster
#[derive(Parser, Debug)]
pub struct ListTopics {
    /// Show detailed topic information
    #[clap(short, long = "verbose")]
    verbose: bool
}

pub fn parse_options() -> Options {
    Options::parse()
}