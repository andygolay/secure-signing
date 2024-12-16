use amazon_kms_app::application::Application;
use amazon_kms_app::hsm::AmazonHsm;
use amazon_kms_app::stream::MockStream;
use amazon_kms_app::message::{Message, Bytes};

#[tokio::main]
async fn main() {
    let region = "us-west-2";
    let key_id = "your-kms-key-id";
    let access_key = "your-access-key";
    let secret_key = "your-secret-key";

    // Initialize HSM
    let hsm = AmazonHsm::new(region, key_id, access_key, secret_key);

    // Simulate a stream of messages
    let actions = vec![
        Message::Sign(Bytes(b"Hello, Amazon KMS!".to_vec())),
        Message::Verify(Bytes(b"Hello, Amazon KMS!".to_vec()), Bytes(vec![/* Signature bytes */])),
    ];
    let stream = MockStream::new(actions);

    // Run the application
    let mut app = Application::new(Box::new(hsm), Box::new(stream));
    app.run().await;
}
