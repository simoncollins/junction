use clap::{ArgSettings, Parser};

/// Read messages from topics
#[derive(Parser, Debug)]
pub struct Push {
    /// The topics to read from (comma delimited)
    #[clap(short, long = "topics", value_name = "TOPICS", setting(ArgSettings::UseValueDelimiter))]
    topic_names: Vec<String>
}