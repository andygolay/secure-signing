use sample_vaultrs_app::application::Application;
use sample_vaultrs_app::hsm::VaultHsm;
use sample_vaultrs_app::stream::MockStream;
use sample_vaultrs_app::message::{Message, Bytes};

#[tokio::main]
async fn main() {
    let vault_addr = "https://<your-hcp-vault-url>";
    let token = "<your-hcp-vault-token>";
    let key_name = "my-key";

    // Initialize HSM
    let hsm = VaultHsm::new(vault_addr, token, key_name);

    // Simulate a stream of messages
    let actions = vec![
        Message::Sign(Bytes(b"Hello, Vault!".to_vec())),
        Message::Verify(Bytes(b"Hello, Vault!".to_vec()), Bytes(vec![/* Signature bytes */])),
    ];
    let stream = MockStream::new(actions);

    // Run the application
    let mut app = Application::new(Box::new(hsm), Box::new(stream));
    app.run().await;
}

