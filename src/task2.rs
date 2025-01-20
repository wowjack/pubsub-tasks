use crate::{task1::TopicManager, task3::Channel, ClientID, TopicID, CLIENT_ID};



// Our TopicManager isn't useful unless we're actually able to send and receive messages.
// Define two variants of the Message enum to represent Publish and Subscribe messages.
// Make sure to include the information required in each message. For instance a subscribe message must contain a client ID and a Topic ID.

pub enum Message {
    Publish {
        src_id: ClientID,
        dst_id: ClientID,
        topic: TopicID,
        data: Vec<u8>,
    },
    Subscribe {
        src_id: ClientID,
        dst_id: ClientID,
        topic: TopicID,
    },
    Unsubscribe {
        src_id: ClientID,
        dst_id: ClientID,
        topic: TopicID,
    }
}


// We have pre-defined a Channel struct with `send` and `receive` methods that you can use to send and receive the above Message types.
// Create a PubSubBroker struct containing an instance of a Channel and a TopicManager

// Define a `handle_message` method for the PubSubBroker.
// This method should take a Message instance as an argument and will perform a side effect based on the message type.
// If it is a publish message, the manager should send the message to everyone subscribed to its topic.
// If it is a subscribe message, the manager should subscribe the source client id to the topic.
// If it is an unsubscribe message, the manager should unsubscribe the source client id from the topic.

// Define a `run` method for the PubSubBroker.
// This method will continuously read messages from the channel, handle any errors, and send the messages to the handle_message method.

pub struct PubSubBroker {
    pub channel: Channel,
    pub subscribers: TopicManager,
}
impl PubSubBroker {
    pub fn handle_message(&mut self, message: Message) {
        match message {
            Message::Publish { topic, data, .. } => {
                // Send message to subscribers
                for sub in self.subscribers.get_subscribers(&topic) {
                    let publish = Message::Publish {
                        src_id: CLIENT_ID,
                        dst_id: sub,
                        topic: topic.clone(),
                        data: data.to_vec()
                    };
                    self.channel.send(publish);
                }
            },
            Message::Subscribe { src_id, topic, .. } => {
                // Subscribe the client to the topic
                self.subscribers.subscribe(&topic, src_id);
            },
            Message::Unsubscribe { src_id, topic, .. } => {
                // Unsubscribe the client from the topic
                self.subscribers.unsubscribe(&topic, src_id);
            },
        }
    }

    pub fn run(mut self) {
        loop {
            match self.channel.receive() {
                Err(reason) => println!("{reason}"),
                Ok(message) => self.handle_message(message),
            }
        }
    }
}




