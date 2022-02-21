mod cli;
mod topics;
mod pull;
mod push;

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
use cli::*;
use cli::topics::*;

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
