use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<Message>
}

pub struct Message {
    pub user: bool,
    pub text: String,
}