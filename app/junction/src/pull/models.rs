use crate::models::Broker;

#[derive(Debug)]
pub struct PullConfig {
    topic_names: String,
    brokers: Vec<Broker>
}