use std::collections::HashMap;

use kanal::{AsyncReceiver, AsyncSender};

#[derive(Debug)]
pub struct PubSub<T> {
    pub connections: HashMap<String, AsyncSender<T>>,
}

impl<T> PubSub<T> {
    pub fn new() -> PubSub<T> {
        PubSub {
            connections: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, producer: AsyncSender<T>) {
        self.connections.insert(name, producer);
    }
}

impl<T: Clone> PubSub<T> {
    pub async fn broadcast(&mut self, message: T) {
        for connection in self.connections.values() {
            connection.send(message.clone()).await.unwrap();
        }
    }
}
