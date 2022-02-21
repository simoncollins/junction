use clap::{Parser};

/// Manage topics
#[derive(Parser, Debug)]
pub struct Topics {
    #[clap(subcommand)]
    pub sub_cmd: TopicsSubCommand,
}

#[derive(Parser, Debug)]
pub enum TopicsSubCommand {
    #[clap()]
    Delete(DeleteTopic),
    List(ListTopics),
    Create(CreateTopic)
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