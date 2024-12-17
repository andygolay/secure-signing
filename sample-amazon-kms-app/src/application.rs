use crate::hsm::Hsm;
use crate::stream::ActionStream;
use crate::message::{Message, Signature};

pub struct Application {
    pub hsm: Box<dyn Hsm + Send + Sync>,
    pub stream: Box<dyn ActionStream + Send>,
}

impl Application {
    pub fn new(hsm: Box<dyn Hsm + Send + Sync>, stream: Box<dyn ActionStream + Send>) -> Self {
        Application { hsm, stream }
    }

    pub async fn run(&mut self) {
        while let Some(message) = self.stream.next().await {
            match message {
                // Handle signing
                Message::Sign(message) => {
                    let signature = self.hsm.sign(message.clone()).await;
                    println!("Signed message: {:?}", signature.0);
    
                    // Immediately verify using the generated signature
                    let verified = self
                        .hsm
                        .verify(message, Signature(signature)) 
                        .await;
    
                    println!("Verified message: {:?}", verified);
                }
                // Handle verification
                Message::Verify(message, signature) => {
                    let verified = self.hsm.verify(message, signature).await;
                    println!("Verified message: {:?}", verified);
                }
            }
        }
    }    
}
