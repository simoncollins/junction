use clap::{ArgSettings, Parser};

/// Write data to a topic
#[derive(Parser, Debug)]
pub struct Push {
    /// The topic to push data to
    #[clap(short, long = "topic", value_name = "TOPIC")]
    topic_name: String
}