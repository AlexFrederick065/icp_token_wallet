use ic_cdk::println; // For logging
use ic_cdk_macros::{query, update};
use candid::{CandidType, Deserialize};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Wallet {
    pub balances: HashMap<String, u64>, // Map of addresses to token balances
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            balances: HashMap::new(),
        }
    }

    pub fn send_tokens(&mut self, to_address: String, amount: u64) -> Result<(), String> {
        let sender_balance = self.balances.entry("self".to_string()).or_insert(0);
        if *sender_balance < amount {
            return Err("Insufficient funds".to_string());
        }
        *sender_balance -= amount;

        let recipient_balance = self.balances.entry(to_address).or_insert(0);
        *recipient_balance += amount;

        Ok(())
    }

    pub fn receive_tokens(&mut self, from_address: String, amount: u64) {
        let recipient_balance = self.balances.entry("self".to_string()).or_insert(0);
        *recipient_balance += amount;

        ic_cdk::println!("Received {} tokens from {}", amount, from_address);
    }

    pub fn get_balance(&self) -> u64 {
        *self.balances.get("self").unwrap_or(&0)
    }
}

// Thread-safe global Wallet instance
static WALLET: Lazy<Mutex<Wallet>> = Lazy::new(|| Mutex::new(Wallet::new()));

#[query]
fn get_balance() -> u64 {
    let wallet = WALLET.lock().expect("Failed to acquire wallet lock");
    wallet.get_balance()
}

#[update]
fn send_tokens(to_address: String, amount: u64) -> Result<(), String> {
    let mut wallet = WALLET.lock().expect("Failed to acquire wallet lock");
    wallet.send_tokens(to_address, amount)
}

#[update]
fn receive_tokens(from_address: String, amount: u64) {
    let mut wallet = WALLET.lock().expect("Failed to acquire wallet lock");
    wallet.receive_tokens(from_address, amount);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wallet() {
        let wallet = Wallet::new();
        assert_eq!(wallet.balances.len(), 0);
    }

    #[test]
    fn test_receive_tokens() {
        let mut wallet = Wallet::new();
        wallet.receive_tokens("sender_address".to_string(), 100);
        assert_eq!(wallet.get_balance(), 100);
    }

    #[test]
    fn test_send_tokens() {
        let mut wallet = Wallet::new();
        wallet.receive_tokens("sender_address".to_string(), 100);
        assert!(wallet.send_tokens("receiver_address".to_string(), 50).is_ok());
        assert_eq!(wallet.get_balance(), 50);
        assert_eq!(*wallet.balances.get("receiver_address").unwrap(), 50);
    }

    #[test]
    fn test_insufficient_funds() {
        let mut wallet = Wallet::new();
        wallet.receive_tokens("sender_address".to_string(), 10);
        assert!(wallet.send_tokens("receiver_address".to_string(), 50).is_err());
    }
}
