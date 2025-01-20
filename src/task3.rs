use std::sync::mpsc;

use crate::task2::{Message, PubSubBroker};

// Finally, we need to actually implement the communication channel so the broker can send and receive messages.

// The channel should use multi producer single consumer queues to send and receive messages.

/// Communication channel that pubsub broker will use
pub struct Channel {
    pub receiver: mpsc::Receiver<Message>,
    pub sender: mpsc::Sender<Message>,
}
impl Channel {
    /// Send a Publish packet to recipient
    pub fn send(&self, message: Message) -> Result<(), String> {
        self.sender.send(message).or(Err("Send Error".to_string()))
    }

    /// Wait to receive a message from the channel
    pub fn receive(&self) -> Result<Message, String> {
        self.receiver.recv().or(Err("Receive Error".to_string()))
    }

    pub fn new() -> (Self, Self) {
        let (s1, r1) = mpsc::channel();
        let (s2, r2) = mpsc::channel();
        (
            Self { receiver: r1, sender: s2 },
            Self { receiver: r2, sender: s1 }
        )
    }
}