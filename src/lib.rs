mod task1;
mod task2;
mod task3;

pub type ClientID = u64;
pub type TopicID = String;

pub const CLIENT_ID: ClientID = 0;


/*
You're helping to build a control system for an autonomous spacecraft.
The spacecraft has many individual subsystems that produce and consume data.The engineering team decided a PubSub messaging model would be the best fit for this situation.

Your system needs to manage a set of topics and subscribers. Subscribers can publish data to a topic or subscribe to a topic. Whenever data is published to a topic, everyone who subscribed to
that topic should be sent the data.
*/

// Tasks:
//  1. Managing data in a collection. What data structure/collection to use? Accessing and mutating data in the collection.
//
//  2. Borrowing and passing data around, control flow. Meant to make you think about ownership and borrowing.
//
//  3. Rust standard library usage.