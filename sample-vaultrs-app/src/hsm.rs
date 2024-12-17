use async_trait::async_trait;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::transit::data::{sign, verify};
use crate::message::{Bytes, Signature};

#[async_trait]
pub trait Hsm {
    async fn sign(&self, message: Bytes) -> Bytes;
    async fn verify(&self, message: Bytes, signature: Signature) -> bool;
}

pub struct VaultHsm {
    client: VaultClient, // Wrap the client in Arc
    key_name: String,
}

impl VaultHsm {
    pub fn new(vault_addr: &str, token: &str, key_name: &str) -> Self {
        // Create the VaultClientSettings
        let settings = VaultClientSettingsBuilder::default()
            .address(vault_addr)
            .token(token)
            .verify(true) // Enable TLS certificate verification
            .build()
            .expect("Failed to build VaultClientSettings");

        // Create the VaultClient
        let client = VaultClient::new(settings).expect("Failed to create VaultClient");

        VaultHsm {
            client,
            key_name: key_name.to_string(),
        }
    }
}

#[async_trait]
impl Hsm for VaultHsm {
    async fn sign(&self, message: Bytes) -> Bytes {
        let input = hex::encode(message.0);
        let result = sign(
                &self.client,
                &self.key_name,
                &input,
                "",        // Empty string as the context
                None,      // No additional options
            )
            .await
            .expect("Failed to sign message");

        Bytes(hex::decode(result.signature).expect("Failed to decode signature"))
    }

    async fn verify(&self, message: Bytes, signature: Signature) -> bool {
        let input = hex::encode(message.0);
        let sig_hex = hex::encode(signature.0);
    
        let response = verify(
            &self.client,
            &self.key_name,
            &input,
            "",
            None,
        )
        .await
        .expect("Failed to verify message");
    
        response.valid // Return the `valid` field from the response
    }
}
