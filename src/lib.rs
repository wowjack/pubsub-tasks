use std::{collections::HashMap, time::Duration};

type HostID = String;
type Topic = String;
/// Map of hosts that have subscribed to a topic
type TopicMap = HashMap<Topic, Vec<HostID>>;

const HOST: &str = "1234abcd";

#[derive(Clone)]
pub enum Message {
    Publish {
        src_host_id: HostID,
        dst_host_id: HostID,
        topic: Topic,
        payload: Vec<u8>,
    },
    Subscribe {
        src_host_id: HostID,
        dst_host_id: HostID,
        topic: Topic,
    }
}


/// Communication channel trait that pubsub host will use
pub trait PubSubChannel {
    fn send(&self, message: Message) -> Result<(), String>;

    /// Blocking
    fn receive(&self) -> Result<Message, String>;

    /// Non blocking
    fn try_receive(&self, timeout: Duration) -> Result<Message, String>;
}


pub struct PubSub<C: PubSubChannel> {
    channel: C,
    subscribers: TopicMap,
}
impl<C: PubSubChannel> PubSub<C> {
    pub fn new(channel: C) -> Self {
        Self {
            channel,
            subscribers: HashMap::new(),
        }
    }

    /// Publish a message to a specific topic
    pub fn publish(&self, topic: &Topic, payload: &[u8]) {
        let Some(subscribers) = self.subscribers.get(topic) else { return };
        for subscriber in subscribers {
            let msg = Message::Publish { src_host_id: HOST.into(), dst_host_id: subscriber.into(), topic: topic.into(), payload: payload.into() };
            self.channel.send(msg);
        }
    }

    /// Subscribe to a specific topic at a specific end host
    pub fn subscribe(&self, topic: &Topic, dst_host: &HostID) {
        let msg = Message::Subscribe { src_host_id: HOST.into(), dst_host_id: dst_host.into(), topic: topic.into() };
        self.channel.send(msg);
    }

    /// handle all subscribe messages and return publish messages for processing.
    pub fn poll_publish(&mut self) -> Vec<Message> {
        let mut publishes = vec![];
        while let Ok(message) = self.channel.try_receive(Duration::from_millis(10)) {
            match message {
                Message::Publish {..} => publishes.push(message),
                Message::Subscribe { src_host_id, dst_host_id, topic } => {
                    self.subscribers.entry(topic).or_insert(vec![]).push(src_host_id);
                },
            }
        }

        publishes
    }
}

