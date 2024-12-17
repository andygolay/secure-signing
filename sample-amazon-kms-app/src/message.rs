#[derive(Clone)]
pub struct Bytes(pub Vec<u8>);

pub struct Signature(pub Bytes);

pub enum Message {
    Sign(Bytes),
    Verify(Bytes, Signature)
}