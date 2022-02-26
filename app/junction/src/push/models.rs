use crate::models::Broker;

#[derive(Debug)]
pub struct PushConfig {
    topic_name: String,
    brokers: Vec<Broker>
}