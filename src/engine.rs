use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io;

use crate::entities::{Client, Transaction};
use crate::utils;

pub fn run(file_name: &str) -> Result<(), Box<dyn Error>> {
    let transactions: Vec<Transaction> = utils::read_csv(file_name)?;

    validate_transactions(&transactions)?;

    let deposits = fetch_deposits(&transactions);
    let mut disputed_transactions: HashSet<u32> = HashSet::new();

    let mut clients: HashMap<u16, Client> = HashMap::new();

    for tx in &transactions {
        let client = clients
            .entry(tx.client_id)
            .or_insert(Client::new(tx.client_id));

        match tx.operation.as_str() {
            "deposit" => {
                if tx.amount > 0.0 {
                    client.credit(tx.amount);
                }
            }
            "withdrawal" => {
                if tx.amount > 0.0 {
                    client.debit(tx.amount);
                }
            }
            "dispute" => {
                // Check deposit exists
                if let Some(&disputed_tx) = deposits.get(&tx.tx_id) {
                    if !disputed_transactions.contains(&disputed_tx.tx_id)  // duplicate check
                        // dispute amount
                        && client.hold(disputed_tx.amount)
                    {
                        disputed_transactions.insert(disputed_tx.tx_id); // mark transaction as disputed.
                    }
                }
            }
            "resolve" => {
                if let Some(&disputed_tx) = deposits.get(&tx.tx_id) {
                    // Check transaction is disputed.
                    if disputed_transactions.contains(&disputed_tx.tx_id)
                        // resolve dispute
                        && client.release_hold(disputed_tx.amount)
                    {
                        disputed_transactions.remove(&disputed_tx.tx_id); // remove dispute.
                    }
                }
            }
            "chargeback" => {
                if let Some(&disputed_tx) = deposits.get(&tx.tx_id) {
                    if disputed_transactions.contains(&disputed_tx.tx_id)
                        && client.charge_back(disputed_tx.amount)
                    {
                        disputed_transactions.remove(&disputed_tx.tx_id);
                    }
                }
            }
            _ => continue,
        }
    }

    utils::write_csv_stdout(&Vec::from_iter(clients.values()))?;

    Ok(())
}

fn validate_transactions(transactions: &Vec<Transaction>) -> Result<(), Box<dyn Error>> {
    /* If there is no data, it could be due to format issues.
    Consider format issues and absent data as invalid data. */
    if transactions.len() == 0 {
        return Err(Box::from(io::Error::from(io::ErrorKind::InvalidData)));
    }

    Ok(())
}

fn fetch_deposits(transactions: &Vec<Transaction>) -> HashMap<u32, &Transaction> {
    let mut transactions_hash: HashMap<u32, &Transaction> = HashMap::new();

    for tx in transactions {
        match tx.operation.as_str() {
            "deposit" => transactions_hash.insert(tx.tx_id, tx),
            _ => continue,
        };
    }

    transactions_hash
}
