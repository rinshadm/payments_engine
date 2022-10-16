use crate::entities::{Client, Transaction};
use crate::utils;
use csv::DeserializeRecordsIntoIter;
use std::collections::{HashMap, HashSet};
use std::error::Error;

pub fn run(file_name: &str) -> Result<(), Box<dyn Error>> {
    let deserialize_iter: DeserializeRecordsIntoIter<_, Transaction> = utils::read_csv(file_name)?;

    let mut deposits: HashMap<u32, f64> = HashMap::new();
    let mut disputed_transactions: HashSet<u32> = HashSet::new();
    let mut clients: HashMap<u16, Client> = HashMap::new();

    for iter in deserialize_iter {
        let tx = iter?;

        let client = clients
            .entry(tx.client_id)
            .or_insert(Client::new(tx.client_id));

        match tx.operation.as_str() {
            "deposit" => {
                if tx.amount > 0.0 {
                    client.credit(tx.amount);
                    deposits.insert(tx.tx_id, tx.amount);
                }
            }
            "withdrawal" => {
                if tx.amount > 0.0 {
                    client.debit(tx.amount);
                }
            }
            "dispute" => {
                // Check deposit exists
                if let Some(&disputed_amount) = deposits.get(&tx.tx_id) {
                    if !disputed_transactions.contains(&tx.tx_id)  // duplicate check
                        // dispute amount
                        && client.hold(disputed_amount)
                    {
                        disputed_transactions.insert(tx.tx_id); // mark transaction as disputed.
                    }
                }
            }
            "resolve" => {
                if let Some(&disputed_amount) = deposits.get(&tx.tx_id) {
                    // Check transaction is disputed.
                    if disputed_transactions.contains(&tx.tx_id)
                        // resolve dispute
                        && client.release_hold(disputed_amount)
                    {
                        disputed_transactions.remove(&tx.tx_id); // remove dispute.
                    }
                }
            }
            "chargeback" => {
                if let Some(&disputed_amount) = deposits.get(&tx.tx_id) {
                    if disputed_transactions.contains(&tx.tx_id)
                        && client.charge_back(disputed_amount)
                    {
                        disputed_transactions.remove(&tx.tx_id);
                    }
                }
            }
            _ => continue,
        }
    }

    utils::write_csv_stdout(&Vec::from_iter(clients.values()))?;

    Ok(())
}