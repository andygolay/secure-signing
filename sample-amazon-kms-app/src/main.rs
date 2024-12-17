use dotenv::dotenv;
use std::env;
use amazon_kms_app::application::Application;
use amazon_kms_app::hsm::AmazonHsm;
use amazon_kms_app::stream::MockStream;
use amazon_kms_app::message::{Message, Bytes, Signature};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let region = env::var("AWS_REGION").expect("AWS_REGION not set");
    let key_id = env::var("AWS_KEY_ID").expect("AWS_KEY_ID not set");
    let access_key = env::var("AWS_ACCESS_KEY").expect("AWS_ACCESS_KEY not set");
    let secret_key = env::var("AWS_SECRET_KEY").expect("AWS_SECRET_KEY not set");

    // Initialize HSM
    let hsm = AmazonHsm::new(&region, &key_id, &access_key, &secret_key);

    // Simulate a stream of messages
    let actions = vec![
        Message::Sign(Bytes(b"Hello, Amazon KMS!".to_vec())),
        Message::Verify(
            Bytes(b"Hello, Amazon KMS!".to_vec()),
            Signature(Bytes(vec![48, 70, 2, 33, 0, 130, 95, 216, 12, 48, 184, 77, 134, 115, 153, 76, 155, 206, 25, 179, 108, 86, 138, 116, 37, 197, 168, 200, 119, 42, 76, 252, 158, 245, 51, 77, 106, 2, 33, 0, 139, 74, 94, 65, 18, 127, 239, 244, 209, 80, 109, 174, 4, 255, 179, 247, 173, 167, 99, 88, 33, 33, 175, 86, 63, 220, 79, 123, 51, 150, 190, 253])),
        ),    ];
    let stream = MockStream::new(actions);

    // Run the application
    let mut app = Application::new(Box::new(hsm), Box::new(stream));
    app.run().await;
}
