mod cli;
mod topics;
mod pull;
mod push;
mod models;

use cli::*;
use cli::topics::*;

#[tokio::main]
async fn main() {
    let opts: cli::Options = cli::parse_options();

    println!("Options: {:?}", opts);

    match &opts.sub_cmd {
        RootSubCommand::Topics(topic_opts) => {
            println!("Manage topics: {:?}", topic_opts);

            match &topic_opts.sub_cmd  {
                TopicsSubCommand::Create(create_topic_opts) => {
                    println!("Create topic: {:?}", create_topic_opts);
                }
                TopicsSubCommand::List(list_topic_opts) => {
                    println!("List topics: {:?}", list_topic_opts);
                }
                TopicsSubCommand::Delete(delete_topic_opts) => {
                    println!("Delete topics: {:?}", delete_topic_opts);
                }
            }
        }
        RootSubCommand::Pull(pull_opts) => {
            println!("Pull data from topic: {:?}", pull_opts);
        }
        RootSubCommand::Push(push_opts) => {
            println!("Push data to topic: {:?}", push_opts);
        }
    }

    let topic = "simontest";
    let brokers = "localhost:9094";
    let topics = vec![topic];
    let group_id = "testgroup";



}
