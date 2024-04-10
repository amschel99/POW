use crate::message::Message;
struct Block {
    hash: String,
    previous: String,
    message: Message,
    nonce: Option<String>,
}
