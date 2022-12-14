use crate::utils::{set_precision_to_four, trim_lowercase};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Transaction {
    #[serde(rename(deserialize = "type"), deserialize_with = "trim_lowercase")]
    pub operation: String,

    #[serde(rename(deserialize = "client"), deserialize_with = "trim_lowercase")]
    pub client_id: u16,

    #[serde(rename(deserialize = "tx"), deserialize_with = "trim_lowercase")]
    pub tx_id: u32,

    #[serde(deserialize_with = "trim_lowercase")]
    pub amount: f64,
}

#[derive(Serialize)]
pub struct Client {
    #[serde(rename = "client")]
    pub client_id: u16,

    #[serde(serialize_with = "set_precision_to_four")]
    available: f64,

    #[serde(serialize_with = "set_precision_to_four")]
    held: f64,

    #[serde(serialize_with = "set_precision_to_four")]
    total: f64,

    locked: bool,
}

impl Client {
    pub fn new(client_id: u16) -> Self {
        Client {
            client_id: client_id,
            available: 0f64,
            held: 0f64,
            total: 0f64,
            locked: false,
        }
    }

    pub fn credit(&mut self, amount: f64) -> bool {
        // If account is locked, don't proceed
        if self.locked {
            return false;
        }

        self.available += amount;
        self.total += amount;

        true
    }

    pub fn debit(&mut self, amount: f64) -> bool {
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

    pub fn hold(&mut self, amount: f64) -> bool {
        if self.locked {
            return false;
        }

        self.held += amount;
        self.available -= amount;

        true
    }

    pub fn release_hold(&mut self, amount: f64) -> bool {
        self.held -= amount;
        self.available += amount;

        true
    }

    pub fn charge_back(&mut self, amount: f64) -> bool {
        self.held -= amount;
        self.total -= amount;
        self.locked = true;

        true
    }
}
