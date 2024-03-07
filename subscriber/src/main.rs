use google_cloud_pubsub::client::{Client, ClientConfig};
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::subscription::SubscriptionConfig;
use google_cloud_gax::grpc::Status;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
// use futures_util::StreamExt;

async fn run(config: ClientConfig) -> Result<(), Status> {

    // Create pubsub client.
    let client = Client::new(config).await.unwrap();

    // Get the topic to subscribe to.
    let topic = client.topic("test-topic");

    // Create subscription
    // If subscription name does not contain a "/", then the project is taken from client above. Otherwise, the
    // name will be treated as a fully qualified resource name
    let config = SubscriptionConfig {
        // Enable message ordering if needed (https://cloud.google.com/pubsub/docs/ordering)
        enable_message_ordering: true,
        ..Default::default()
    };

    // Create subscription
    let subscription = client.subscription("test-subscription");
    if !subscription.exists(None).await? {
        subscription.create(topic.fully_qualified_name(), config, None).await?;
    }

    // Token for cancel.
    let cancel = CancellationToken::new();
    let cancel2 = cancel.clone();
    tokio::spawn(async move {
        // Cancel after 10 seconds.
        tokio::time::sleep(Duration::from_secs(1000)).await;
        cancel2.cancel();
    });

    // Receive blocks until the ctx is cancelled or an error occurs.
    // Or simply use the `subscription.subscribe` method.
    subscription.receive(|mut message, cancel| async move {
        // Handle data.
        println!("Got Message: {:?}", message.message.data);

        // Ack or Nack message.
        let _ = message.ack().await;
    }, cancel.clone(), None).await?;

    // Delete subscription if needed.
    subscription.delete(None).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let config = ClientConfig::default().with_auth().await.unwrap();
    println!("config.environment: {:?}", config.environment);
    // let client = Client::new(config).await.unwrap();
    let _ = run(config).await;
}
