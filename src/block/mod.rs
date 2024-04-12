use crate::message::Message;
use std::cell::RefCell;
use std::collections::LinkedList;

thread_local! {
    static  BLOCKCHAIN: RefCell<LinkedList<Block>> = RefCell::new(LinkedList::new());
}
#[derive(Debug)]
struct Block {
    hash: String,
    previous: Option<String>,
    message: Option<Message>,
    nonce: Option<String>,
    metadata: Option<String>,
}
pub trait IsBlock {
    fn unique_hash() -> String;
}

impl IsBlock for Block {
    fn unique_hash() -> String {
        String::from("This is a random unique hash")
    }
}

impl Block {
    fn genesis_block() -> Block {
        Block {
            hash: Block::unique_hash(),
            previous: None,
            message: None,
            nonce: None,
            metadata: Some(String::from("Know thyself")),
        }
    }
}

pub fn launch_blockchain() {
    BLOCKCHAIN.with(|blockchain| blockchain.borrow_mut().push_front(Block::genesis_block()));
}
#[cfg(test)]
mod tests {
    use std::clone;

    use super::*;

    #[test]
    fn test_launch_blockchain() {
        launch_blockchain();
        let chain =
            BLOCKCHAIN.with(|blockchain: &RefCell<LinkedList<Block>>| blockchain.take().pop_back());
        dbg!(chain);
    }
}
