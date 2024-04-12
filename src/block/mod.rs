use crate::message::Message;
use std::cell::RefCell;
use std::collections::LinkedList;

thread_local! {
    static  BLOCKCHAIN: RefCell<Blockchain> = RefCell::new(Blockchain::new());
}
#[derive(Debug, Clone)]
struct Block {
    hash: String,
    previous: Option<String>,
    message: Option<Message>,
    nonce: Option<String>,
    metadata: Option<String>,
}
#[derive(Debug)]
struct Blockchain {
    blocks: LinkedList<Block>,
    difficulty: u64,
}
impl Blockchain {
    fn new() -> Self {
        Self {
            blocks: LinkedList::new(),
            difficulty: 1,
        }
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push_back(block);
    }

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

pub trait IsBlock {
    fn unique_hash() -> String;
}

impl IsBlock for Block {
    fn unique_hash() -> String {
        String::from("This is a random unique hash")
    }
}

pub fn launch_blockchain() {
    BLOCKCHAIN.with(|blockchain| {
        blockchain
            .borrow_mut()
            .blocks
            .push_back(Blockchain::genesis_block())
    });
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_launch_blockchain() {
        launch_blockchain();
        let blocks = BLOCKCHAIN.with(|blockchain| blockchain.borrow_mut().blocks.clone());
        let difficulty = BLOCKCHAIN.with(|blockchain| blockchain.borrow_mut().difficulty.clone());
        dbg!(blocks);
        dbg!(difficulty);
    }
}
