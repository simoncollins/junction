use clap::{ArgSettings, Parser};

/// Send messages to a topic
#[derive(Parser, Debug)]
pub struct Pull {
    /// The topics to send data to
    #[clap(short, long = "topics", value_name = "TOPICS", setting(ArgSettings::UseValueDelimiter))]
    topic_names: Vec<String>
}