// First we need to know who is subscribed to each topic.
// Design a struct TopicManager to manage a list of topic and who is subscribed to them.

// It should have subscribe and unsubscribe methods that take a topic id and subscriber id.
// It should also have a get_subscribers method to get the list of subscribers to a certain topic.


use std::collections::{HashMap, HashSet};

use crate::{ClientID, TopicID};

pub struct TopicManager {
    pub subscribers: HashMap<TopicID, HashSet<ClientID>>,
}
impl TopicManager {
    /// Add a subscriber to the subscriber list for a topic
    pub fn subscribe(&mut self, topic: &TopicID, subscriber: ClientID) {
        self.subscribers.entry(topic.into()).or_default().insert(subscriber);
    }

    /// Remove a subscriber from the subscriber list for a topic
    pub fn unsubscribe(&mut self, topic: &TopicID, subscriber: ClientID) {
        if let Some(subs) = self.subscribers.get_mut(topic) {
            subs.remove(&subscriber);
        }
    }

    /// Get the list of subscribers to a specific topic
    pub fn get_subscribers(&self, topic: &TopicID) -> Vec<ClientID> {
        self.subscribers
            .get(topic)
            .map_or(
                vec![],
                |subs| subs.iter().cloned().collect()
            )
    }
}