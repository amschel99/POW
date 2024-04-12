#[derive(Debug)]
pub struct Message {
    sender: String,
    receiver: String,
    amount: u128,
    timestamp: String,
    signature: String,
    metadata: Option<String>,
}
