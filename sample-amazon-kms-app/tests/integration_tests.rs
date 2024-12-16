#[tokio::test]
async fn test_application_run() {
    use amazon_kms_app::application::Application;
    use amazon_kms_app::hsm::AmazonHsm;
    use amazon_kms_app::stream::MockStream;
    use amazon_kms_app::message::{Message, Bytes};

    let region = "us-west-2";
    let key_id = "test-key-id";
    let access_key = "test-access-key";
    let secret_key = "test-secret-key";

    let hsm = AmazonHsm::new(region, key_id, access_key, secret_key);
    let actions = vec![Message::Sign(Bytes(b"Hello, test!".to_vec()))];
    let stream = MockStream::new(actions);

    let mut app = Application::new(Box::new(hsm), Box::new(stream));
    app.run().await;
}
