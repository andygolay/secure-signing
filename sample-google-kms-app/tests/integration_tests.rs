#[tokio::test]
async fn test_application_run() {
    use sample_vaultrs_app::application::Application;
    use sample_vaultrs_app::hsm::VaultHsm;
    use sample_vaultrs_app::stream::MockStream;
    use sample_vaultrs_app::message::{Message, Bytes};

    let vault_addr = "https://<your-hcp-vault-url>";
    let token = "<your-hcp-vault-token>";
    let key_name = "test-key";

    let hsm = VaultHsm::new(vault_addr, token, key_name);
    let actions = vec![Message::Sign(Bytes(b"Hello, test!".to_vec()))];
    let stream = MockStream::new(actions);

    let mut app = Application::new(Box::new(hsm), Box::new(stream));
    app.run().await;
}
