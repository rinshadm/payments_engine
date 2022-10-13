use std::io;
use std::error::Error;
use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use crate::utils;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename(deserialize = "type"))]
    operation: String,

    #[serde(rename(deserialize = "client"))]
    client_id: u16,

    #[serde(rename(deserialize = "tx"))]
    tx_id: u32,

    amount: f32
}

#[derive(Serialize)]
struct Client {
    #[serde(rename = "client")]
    pub client_id: u16,

    available: f32,

    held: f32,

    total: f32,

    locked: bool
}

impl Client {
    fn new(client_id: u16) -> Self {
        Client {
            client_id: client_id,
            available: 0f32,
            held: 0f32,
            total: 0f32,
            locked: false
        }
    }

    fn credit(&mut self, amount: f32) -> bool {
        // If account is locked, don't proceed
        if self.locked {
            return false;
        }

        self.available += amount;
        self.total += amount;

        true
    }

    fn debit(&mut self, amount: f32) -> bool {
        // If account is locked, don't proceed
        if self.locked {
            return false;
        }

        if self.available >= amount {
            self.available -= amount;
            self.total -= amount;

            return true;
        }

        false
    }

    fn hold(&mut self, amount: f32) -> bool {
        // If account is locked, don't proceed
        if self.locked {
            return false;
        }

        self.held += amount;
        self.available -= amount;

        true
    }

    fn release_hold(&mut self, amount: f32) -> bool {
        self.held -= amount;
        self.available += amount;

        true
    }

    fn charge_back(&mut self, amount: f32) -> bool {
        self.held -= amount;
        self.total -= amount;
        self.locked = true;

        true
    }
}

pub fn run(file_name: &str) -> Result<(), Box<dyn Error>> {
    let transactions: Vec<Transaction> = utils::read_csv(file_name)?;
    
    validate_transactions(&transactions)?;

    let transactions_hash: HashMap<u32, &Transaction> = create_transactions_hash(&transactions);
    let mut disputed_transactions: HashSet<u32> = HashSet::new();
    
    let mut clients: HashMap<u16, Client> = HashMap::new();

    for tx in &transactions {
        let client = clients.entry(tx.client_id)
                                         .or_insert(Client::new(tx.client_id));

        match tx.operation.as_str() {
            "deposit" => {
                if tx.amount > 0.0 {
                    client.credit(tx.amount);
                }
            },
            "withdrawal" => {
                if tx.amount > 0.0 {
                    client.debit(tx.amount);
                }
            },
            "dispute" => {
                // Check transaction exists
                if let Some(&disputed_tx) = transactions_hash.get(&tx.tx_id) {
                    // Check duplicate disputes.
                    if !disputed_transactions.contains(&disputed_tx.tx_id) {
                        // If dispute is successful, mark transaction as disputed.
                        if client.hold(disputed_tx.amount) {
                            disputed_transactions.insert(disputed_tx.tx_id);
                        }
                    }
                } 
            },
            "resolve" => {
                // Check transaction exists
                if let Some(&disputed_tx) = transactions_hash.get(&tx.tx_id) {
                    // Check if transaction is disputed.
                    if disputed_transactions.contains(&disputed_tx.tx_id) {
                        // If successfully resolved, mark transaction as undisputed.
                        if client.release_hold(disputed_tx.amount) {
                            disputed_transactions.remove(&disputed_tx.tx_id);
                        }
                    }
                } 
            },
            "chargeback" => {
                // Check transaction exists
                if let Some(&disputed_tx) = transactions_hash.get(&tx.tx_id) {
                    // Check if transaction is disputed.
                    if disputed_transactions.contains(&disputed_tx.tx_id) {
                        // If successfully charged back, mark transaction as undisputed.
                        if client.charge_back(disputed_tx.amount) {
                            disputed_transactions.remove(&disputed_tx.tx_id);
                        }
                    }
                } 
            }
            _ => continue
        }
    }

    utils::write_csv_stdout(&Vec::from_iter(clients.values()))?;

    Ok(())
}

fn validate_transactions(transactions: &Vec<Transaction>) -> Result<(), Box<dyn Error>> {
    /* If there is no data, it could be due to format issues.
       Consider format issues and absent data as invalid data. */
    if transactions.len() == 0 {
        return Err(
            Box::from(
                io::Error::from(
                    io::ErrorKind::InvalidData)));
    }

    Ok(())
}

fn create_transactions_hash(transactions: &Vec<Transaction>) -> HashMap<u32, &Transaction> {
    let mut transactions_hash: HashMap<u32, &Transaction> = HashMap::new();

    for tx in transactions {
        match tx.operation.as_str() {
            "deposit" | "withdrawal" => transactions_hash.insert(tx.tx_id, tx),
            _ => continue
        };
    }

    transactions_hash
}