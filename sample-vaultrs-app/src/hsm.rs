use async_trait::async_trait;
use base64::{encode, Engine};
use base64::engine::general_purpose;
use vaultrs::api::transit::requests::VerifySignedDataRequest;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::transit::data::{sign, verify};
use crate::message::{Bytes, Signature};

#[async_trait]
pub trait Hsm {
    async fn sign_message(&self, message: Bytes) -> Signature;
    async fn verify_message(&self, message: Bytes, signature: Signature) -> bool;
}

pub struct VaultHsm {
    client: VaultClient, 
    key_name: String,
}

impl VaultHsm {
    pub fn new(vault_addr: &str, token: &str, key_name: &str) -> Self {
        // Create the VaultClientSettings
        let settings = VaultClientSettingsBuilder::default()
            .address(vault_addr)
            .token(token)
            .namespace(Some("admin".to_string()))
            .verify(true) 
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
    async fn sign_message(&self, message: Bytes) -> Signature {
        // Base64-encode the input message
        let input = general_purpose::STANDARD.encode(&message.0);

        // Call the Transit engine's sign function
        let result = sign(
            &self.client,      // Vault client
            "transit",         // Mount point
            &self.key_name,    // Key name
            &input,            // Base64-encoded input
            None,              // No extra options
        )
        .await
        .expect("Failed to sign message");

        // Print the Base64-encoded signature
        println!("Vault signature: {}", result.signature);

        // Return the signature as a string
        Signature(result.signature.into())
    }

    // Verify the message using the prefixed Vault signature
    async fn verify_message(&self, message: Bytes, signature: Signature) -> bool {
        let input = general_purpose::STANDARD.encode(&message.0);

        // Pass the Vault signature (with prefix) directly
        let response = verify(
            &self.client,        // Vault client
            "transit",           // Mount point
            &self.key_name,      // Key name
            &input,              // Base64-encoded input message
            Some(
                VerifySignedDataRequest::builder()
                    .signature(&String::from_utf8(signature.0.clone()).expect("Invalid UTF-8 in signature"))
            ),
        )
        .await
        .expect("Failed to verify message");

        response.valid
    }
}
