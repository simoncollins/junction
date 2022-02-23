mod cli;
mod topics;
mod pull;
mod push;

use rdkafka::util::get_rdkafka_version;
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

    // let (version_n, version_s) = get_rdkafka_version();
    // println!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let topic = "simontest";
    let brokers = "localhost:9094";
    let topics = vec![topic];
    let group_id = "testgroup";

    // produce(brokers, topic).await;
    // consume_and_print(brokers, group_id, &topics).await
}
