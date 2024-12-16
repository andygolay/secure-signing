use async_trait::async_trait;
use google_cloud_kms::client::{Client, ClientConfig};
use google_cloud_kms::grpc::kms::v1::{AsymmetricSignRequest, Digest};
use google_cloud_gax::retry::RetrySetting;
use crate::message::{Bytes, Signature};

#[async_trait]
pub trait Hsm {
    async fn sign(&self, message: Bytes) -> Bytes;
    async fn verify(&self, message: Bytes, signature: Signature) -> bool;
}

pub struct GoogleHsm {
    client: Client,
    key_version_name: String,
}

impl GoogleHsm {
    pub async fn new(key_version_name: &str) -> Self {
        let config = ClientConfig::default();
        let client = Client::new(config)
            .await
            .expect("Failed to create Google KMS client");

        GoogleHsm {
            client,
            key_version_name: key_version_name.to_string(),
        }
    }
}

#[async_trait]
impl Hsm for GoogleHsm {
    async fn sign(&self, message: Bytes) -> Bytes {
        let digest = Digest {
            digest: Some(Digest {
                digest: Some(google_cloud_kms::grpc::kms::v1::digest::Digest::Sha256(message.0)),
            }),
        };

        let request = AsymmetricSignRequest {
            name: self.key_version_name.clone(),
            digest: Some(digest), // Set the digest
            ..Default::default() // Fill unused fields with defaults
        };

        let response = self
            .client
            .asymmetric_sign(request, Some(RetrySetting::default()))
            .await
            .expect("Failed to sign message");

        Bytes(response.signature)
    }

    async fn verify(&self, _message: Bytes, _signature: Signature) -> bool {
        unimplemented!("Verify is not supported directly by Google KMS.");
    }
}
