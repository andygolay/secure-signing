use dotenv::dotenv;
use std::env;

use sample_vaultrs_app::application::Application;
use sample_vaultrs_app::hsm::{Hsm, VaultHsm};
use sample_vaultrs_app::stream::MockStream;
use sample_vaultrs_app::message::{Message, Bytes, Signature};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let vault_addr = env::var("VAULT_ADDR").expect("VAULT_ADDR not set");
    let token = env::var("VAULT_TOKEN").expect("VAULT_TOKEN not set");
    let key_name = env::var("KEY_NAME").expect("KEY_NAME not set");

    let hsm = VaultHsm::new(&vault_addr, &token, &key_name);

    let message_bytes = Bytes(b"Hello, Vault!".to_vec());
    let signature = hsm.sign(message_bytes.clone()).await;

    let actions = vec![
        Message::Sign(message_bytes.clone()),
        Message::Verify(message_bytes, Signature(signature.0))
    ];

    let stream = MockStream::new(actions);
    let mut app = Application::new(Box::new(hsm), Box::new(stream));
    app.run().await;
}
