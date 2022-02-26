use clap::{ArgSettings, Parser};

/// Read data from one or more topics
#[derive(Parser, Debug)]
pub struct Pull {
    /// The topics to pull data from
    #[clap(short, long = "topics", value_name = "TOPICS", setting(ArgSettings::UseValueDelimiter))]
    topic_names: Vec<String>
}