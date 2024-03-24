use std::sync::Arc;

use codecs::json::JsonCodec;
use message::Message;
use pubsub::PubSub;
use socket::Socket;

use kanal;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub mod codec;
pub mod codecs;
pub mod message;
pub mod pubsub;
pub mod socket;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:1422")
        .await
        .expect("Failed to bind at port 1422.");

    let pubsub = Arc::from(Mutex::from(PubSub::<Message>::new()));

    while let Ok((stream, address)) = listener.accept().await {
        let pubsub = pubsub.clone();

        tokio::spawn(async move {
            let mut registry = pubsub.lock().await;
            let codec = JsonCodec::new();

            let mut socket = Socket::new(stream);
            let (producer, consumer) = kanal::unbounded_async::<Message>();

            registry.add(address.to_string(), producer.clone());
            drop(registry);

            let mut read_buffer = [0; 1024];
            loop {
                tokio::select! {
                    outgoing = consumer.recv() => match outgoing {
                        Ok(Message::Hello(msg)) => { socket.send(Message::Hello(msg), &codec).await.unwrap(); },
                        Err(_) => {}
                    },

                    incoming = socket.recv::<Message>(&mut read_buffer, &codec) => match incoming {
                        Ok(Some(message)) => {
                            let mut pubsub = pubsub.lock().await;

                            pubsub.broadcast(message).await;
                        },
                        Err(error) => eprintln!("{:?}", error),
                        Ok(_) => {}
                    }
                }
            }
        });
    }
}
