use google_cloud_gax::grpc::Status;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::{Client, ClientConfig};
use google_cloud_pubsub::subscription::SubscriptionConfig;
use google_cloud_pubsub::topic::TopicConfig;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

async fn run(config: ClientConfig) -> Result<(), Status> {
    // Create pubsub client.
    let client = Client::new(config).await.unwrap();

    // Create topic.
    let topic = client.topic("test-topic");
    print!("topic: {:?}", topic.id());
    if !topic.exists(None).await? {
        topic.create(None, None).await?;
    }

    // Start publisher.
    let publisher = topic.new_publisher(None);

    // Publish message.
    let tasks: Vec<JoinHandle<Result<String, Status>>> = (0..1)
        .into_iter()
        .map(|_i| {
            let publisher = publisher.clone();
            tokio::spawn(async move {
                let msg = PubsubMessage {
                    data: "テイエムオペラオー T.M.Opera O 抹消　牡　栗毛
                    ".into(),
                    // Set ordering_key if needed (https://cloud.google.com/pubsub/docs/ordering)
                    ordering_key: "order".into(),
                    ..Default::default()
                };

                // Send a message. There are also `publish_bulk` and `publish_immediately` methods.
                let mut awaiter = publisher.publish(msg).await;

                // The get method blocks until a server-generated ID or an error is returned for the published message.
                awaiter.get().await
            })
        })
        .collect();

    // Wait for all publish task finish
    for task in tasks {
        let message_id = task.await.unwrap()?;
    }

    // Wait for publishers in topic finish.
    let mut publisher = publisher;
    publisher.shutdown();

    Ok(())
}

#[tokio::main]
async fn main() {
    let config = ClientConfig::default().with_auth().await.unwrap();
    println!("project_id: {:?}", config.project_id);
    println!("config.environment: {:?}", config.environment);
    // let client = Client::new(config).await.unwrap();
    let _ = run(config).await;
}
