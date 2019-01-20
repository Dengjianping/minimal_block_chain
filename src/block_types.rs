use std::{
    hash::{ Hash, Hasher },
    time::SystemTime,
    collections::hash_map::DefaultHasher,
    cmp::{ self, PartialEq, Eq, PartialOrd },
};
use serde_derive::{ Deserialize, Serialize };


fn get_init_hash_by_time() -> String {
    let now = SystemTime::now();
    let mut hasher = DefaultHasher::new();
    now.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}


#[derive(Clone, Debug, Hash, Deserialize, Serialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amout: u32,
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction{
            sender: String::from("jdeng"),
            recipient: String::from("jamie"),
            amout: 0,
        }
    }
}


#[derive(Clone, Debug, Hash, Deserialize, Serialize)]
pub struct Block {
    index: usize,
    timestamp: SystemTime,
    proof: i64,
    prev_hashed: String,
    transaction: Vec<Transaction>,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            index: 0,
            timestamp: SystemTime::now(),
            proof: 0,
            prev_hashed: get_init_hash_by_time(),
            transaction: vec![Transaction::default()],
        }
    }
}


#[derive(Clone, Debug, Hash, Deserialize, Serialize)]
pub struct BlockChain {
    chains: Vec<Block>,
    current_transaction: Vec<Transaction>,
}

impl Default for BlockChain {
    fn default() -> Self {
        BlockChain {
            chains: vec![Block::default()],
            current_transaction: vec![Transaction::default()],
        }
    }
}


trait BlockHash {
    type Output;
    fn block_hash(&self) -> Self::Output;
}

impl BlockHash for Block {
    type Output = String;
    fn block_hash(&self) -> Self::Output {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}


impl BlockChain {
    fn new_transaction(&mut self, sender: &str, recipient: &str, amout: u32) -> usize {
        let tran = Transaction {
            sender: String::from(sender),
            recipient: String::from(recipient),
            amout
        };
        self.current_transaction.push(tran);
        if let Some(block) = self.last_block() {
            block.index + 1
        } else {
            1
        }
    }

    fn add_block(&mut self, proof: i64, prev_hashed: &str) -> Block {
        let new_block = Block {
            index: self.chains.len() + 1,
            timestamp: SystemTime::now(),
            proof,
            // prev_hashed: prev_hashed.to_string(),
            prev_hashed: self.chains[self.chains.len() - 1].block_hash(),
            transaction: self.current_transaction.clone(), // rust cannot move partial elements now
        };
        self.current_transaction.clear(); // clear all transactions
        self.chains.push(new_block.clone());
        new_block
    }

    fn last_block(&self) -> Option<&Block> {
        self.chains.last()
    }

    fn proof_of_work(&self, last_proof: i64) -> i64 {
        let mut proof: i64 = 0;

        loop {
            if self.valid_proof(last_proof, proof) {
                break;
            }
            proof += 1;
        }
        proof
    }

    fn valid_proof(&self, last_proof: i64, proof: i64) -> bool {
        let mut hasher = DefaultHasher::new();
        let guess = format!("{}{}", last_proof, proof);
        guess.hash(&mut hasher);
        let hashed = format!("{:x}", hasher.finish());
        hashed.ends_with("0000") // take 0000 as a proof
    }
}


pub fn trial() {
    let tran = Transaction{
        sender: String::from("jdeng"),
        recipient: String::from("jamie"),
        amout: 20,
    };
    let block = Block {
        index: 0,
        timestamp: SystemTime::now(),
        proof: 100,
        prev_hashed: String::from(""),
        transaction: vec![tran],
    };

    println!("hashed block value: {:?}", block.block_hash());

    let a = 123;
    let b = 1323;
    let c = "123123";
    println!("{:?}", c.as_bytes());
}