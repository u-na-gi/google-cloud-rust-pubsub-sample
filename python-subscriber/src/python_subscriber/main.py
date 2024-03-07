from typing import Optional


def receive_messages(
    project_id: str, subscription_id: str, timeout: Optional[float] = None
) -> None:
    """Receives messages from a pull subscription."""
    # [START pubsub_subscriber_async_pull]
    # [START pubsub_quickstart_subscriber]
    from concurrent.futures import TimeoutError

    from google.cloud import pubsub_v1
    from google.cloud.pubsub_v1.subscriber import exceptions as sub_exceptions

    # TODO(developer)
    # project_id = "your-project-id"
    # subscription_id = "your-subscription-id"
    # Number of seconds the subscriber should listen for messages
    # timeout = 5.0

    subscriber = pubsub_v1.SubscriberClient()
    print("subscriber.api.endpoint: ", subscriber.api.api_endpoint)

    # The `subscription_path` method creates a fully qualified identifier
    # in the form `projects/{project_id}/subscriptions/{subscription_id}`
    subscription_path = subscriber.subscription_path(project_id, subscription_id)
    def callback(message: pubsub_v1.subscriber.message.Message) -> None:
        print(f"Received message id {message.message_id}.")
        print(f"Received {message}.")
        # Use `ack_with_response()` instead of `ack()` to get a future that tracks
        # the result of the acknowledge call. When exactly-once delivery is enabled
        # on the subscription, the message is guaranteed to not be delivered again
        # if the ack future succeeds.
        ack_future = message.ack_with_response()

        try:
            # Block on result of acknowledge call.
            # When `timeout` is not set, result() will block indefinitely,
            # unless an exception is encountered first.
            ack_future.result(timeout=timeout)
            print(f"Ack for message {message.message_id} successful.")
        except sub_exceptions.AcknowledgeError as e:
            print(
                f"Ack for message {message.message_id} failed with error: {e.error_code}"
            )


    streaming_pull_future = subscriber.subscribe(subscription_path, callback=callback)
    print(f"Listening for messages on {subscription_path} ..\n")

    # Wrap subscriber in a 'with' block to automatically call close() when done.
    with subscriber:
        try:
            # When `timeout` is not set, result() will block indefinitely,
            # unless an exception is encountered first.
            streaming_pull_future.result(timeout=timeout)
        except TimeoutError:
            streaming_pull_future.cancel()  # Trigger the shutdown.
            streaming_pull_future.result()  # Block until the shutdown is complete.
    # [END pubsub_subscriber_async_pull]
    # [END pubsub_quickstart_subscriber]

if __name__ == "__main__":
    subscription_id = "test-subscription"
    project_id = "local-project"
    # timeout = 10.0
    receive_messages(project_id=project_id, subscription_id=subscription_id)
    