use clap::{Args, ArgEnum, ArgGroup, AppSettings, ArgSettings, Parser};

use std::time::Duration;

use rdkafka::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::Rebalance;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::util::get_rdkafka_version;

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        println!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        println!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        println!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

async fn consume_and_print(brokers: &str, group_id: &str, topics: &[&str]) {
    let context = CustomContext;

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
            Err(e) => println!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        println!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for i in 0..headers.count() {
                        let header = headers.get(i).unwrap();
                        println!("  Header {:#?}: {:?}", header.0, header.1);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}

async fn produce(brokers: &str, topic_name: &str) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // This loop is non blocking: all messages will be sent one after the other, without waiting
    // for the results.
    let futures = (0..5)
        .map(|i| async move {
            // The send operation on the topic returns a future, which will be
            // completed once the result or failure from Kafka is received.
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic_name)
                        .payload(&format!("Message {}", i))
                        .key(&format!("Key {}", i))
                        .headers(OwnedHeaders::new().add("header_key", "header_value")),
                    Duration::from_secs(0),
                )
                .await;

            // This will be executed when the result is received.
            println!("Delivery status for message {} received", i);
            delivery_status
        })
        .collect::<Vec<_>>();

    // This loop will wait until all delivery statuses have been received.
    for future in futures {
        println!("Future completed. Result: {:?}", future.await);
    }
}

/// Manage Kafka clusters and send + receive messages to + from topics
#[derive(Parser)]
#[clap(version = "0.1", author = "Simon Collins <hello@simoncollins.dev>")]
struct Options {
    /// Boostrap brokers (comma separated) list.
    /// Mandatory if not configured via .junctionrc
    #[clap(short, long)]
    brokers: Option<Vec<String>>,

    #[clap(subcommand)]
    sub_cmd: RootSubCommand,
}

#[derive(Parser)]
enum RootSubCommand {
    #[clap()]
    Topics(Topics),
    Push(Push),
    Pull(Pull)
}

#[derive(Parser)]
enum TopicsSubCommand {
    #[clap()]
    Delete(DeleteTopic),
    List(ListTopics),
    Create(CreateTopic)
}

/// Manage topics
#[derive(Parser)]
struct Topics {
    #[clap(subcommand)]
    sub_cmd: TopicsSubCommand,
}

/// Read messages from topics
#[derive(Parser)]
struct Push {
    /// The topics to read from (comma delimited)
    #[clap(short, long = "topics", value_name = "TOPICS", setting(ArgSettings::UseValueDelimiter))]
    topic_names: Vec<String>
}

/// Send messages to a topic
#[derive(Parser)]
struct Pull {
    /// The topic to send to
    #[clap(short, long = "topic", value_name = "TOPIC")]
    topic_name: String
}

/// Create a new topic
#[derive(Parser)]
struct CreateTopic {
    /// The topic to create
    topic_name: String
}

/// Send messages to a topic
#[derive(Parser)]
struct DeleteTopic {
    /// The topic to delete
    topic_name: String
}

/// List topics on the cluster
#[derive(Parser)]
struct ListTopics {
    /// Show detailed topic information
    #[clap(short, long = "verbose")]
    verbose: bool
}

#[tokio::main]
async fn main() {
    // let opts: Opt = Opt::parse();
    let opts: Options = Options::parse();
    //
    println!("Brokers: {:?}", opts.brokers);

    // let (version_n, version_s) = get_rdkafka_version();
    // println!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let topic = "simontest";
    let brokers = "localhost:9094";
    let topics = vec![topic];
    let group_id = "testgroup";

    // produce(brokers, topic).await;
    consume_and_print(brokers, group_id, &topics).await
}
