use async_trait::async_trait;
use crate::message::Message;

#[async_trait]
pub trait ActionStream {
    async fn next(&mut self) -> Option<Message>;
}

pub struct MockStream {
    actions: Vec<Message>,
}

impl MockStream {
    pub fn new(actions: Vec<Message>) -> Self {
        MockStream { actions }
    }
}

#[async_trait]
impl ActionStream for MockStream {
    async fn next(&mut self) -> Option<Message> {
        if self.actions.is_empty() {
            None
        } else {
            Some(self.actions.remove(0))
        }
    }
}
