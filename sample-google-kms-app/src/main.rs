use google_kms_app::application::Application;
use google_kms_app::hsm::GoogleHsm;
use google_kms_app::stream::MockStream;
use google_kms_app::message::{Message, Bytes};

#[tokio::main]
async fn main() {
    let key_version_name = "projects/<project-id>/locations/global/keyRings/<key-ring>/cryptoKeys/<key-name>/cryptoKeyVersions/1";

    // Initialize Google HSM
    let hsm = GoogleHsm::new(key_version_name).await;

    // Simulate a stream of messages
    let actions = vec![
        Message::Sign(Bytes(b"Hello, Google KMS!".to_vec())),
    ];
    let stream = MockStream::new(actions);

    // Run the application
    let mut app = Application::new(Box::new(hsm), Box::new(stream));
    app.run().await;
}
