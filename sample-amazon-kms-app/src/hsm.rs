use async_trait::async_trait;
use aws_sdk_kms::{Client, Config, Region, Credentials};
use aws_sdk_kms::model::SigningAlgorithmSpec;
use aws_sdk_kms::output::{SignOutput, VerifyOutput};
use aws_sdk_kms::types::Blob;
use std::time::SystemTime;
use crate::message::{Bytes, Signature};

#[async_trait]
pub trait Hsm {
    async fn sign(&self, message: Bytes) -> Bytes;
    async fn verify(&self, message: Bytes, signature: Signature) -> bool;
}

pub struct AmazonHsm {
    client: Client,
    key_id: String,
}

impl AmazonHsm {
    pub fn new(region: &str, key_id: &str, access_key: &str, secret_key: &str) -> Self {
        let region = Region::new(region.to_string());
        let credentials = Credentials::new(
            access_key,
            secret_key,
            None, // No expiration time
            None, // No session token
            "CustomProvider",
        );
        let config = Config::builder()
            .region(region)
            .credentials_provider(credentials)
            .build();

        let client = Client::from_conf(config);

        AmazonHsm {
            client,
            key_id: key_id.to_string(),
        }
    }
}

#[async_trait]
impl Hsm for AmazonHsm {
    async fn sign(&self, message: Bytes) -> Bytes {
        let blob = Blob::new(message.0);
        let request = self
            .client
            .sign()
            .key_id(&self.key_id)
            .signing_algorithm(SigningAlgorithmSpec::EcdsaSha256)
            .message(blob);

        let response: SignOutput = request.send().await.expect("Failed to sign message");

        Bytes(
            response
                .signature()
                .expect("Missing signature")
                .as_ref()
                .to_vec(),
        )
    }

    async fn verify(&self, message: Bytes, signature: Signature) -> bool {
        let message_blob = Blob::new(message.0);
        let signature_blob = Blob::new(signature.0 .0);
        let request = self
            .client
            .verify()
            .key_id(&self.key_id)
            .signing_algorithm(SigningAlgorithmSpec::EcdsaSha256)
            .message(message_blob)
            .signature(signature_blob);
    
        let response: VerifyOutput = request.send().await.expect("Failed to verify signature");
    
        response.signature_valid()
    }
}
