use dotenv::dotenv;
use std::env;
use amazon_kms_app::application::Application;
use amazon_kms_app::hsm::AmazonHsm;
use amazon_kms_app::stream::MockStream;
use amazon_kms_app::message::{Message, Bytes};

#[tokio::test]
async fn test_application_run() {
    dotenv::dotenv().ok();

    let region = env::var("AWS_REGION").expect("AWS_REGION not set");
    let key_id = env::var("AWS_KEY_ID").expect("AWS_KEY_ID not set");
    let access_key = env::var("AWS_ACCESS_KEY").expect("AWS_ACCESS_KEY not set");
    let secret_key = env::var("AWS_SECRET_KEY").expect("AWS_SECRET_KEY not set");


    let hsm = AmazonHsm::new(&region, &key_id, &access_key, &secret_key);
    let actions = vec![Message::Sign(Bytes(b"Hello, test!".to_vec()))];
    let stream = MockStream::new(actions);

    let mut app = Application::new(Box::new(hsm), Box::new(stream));
    app.run().await;
}
