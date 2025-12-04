use leptos::prelude::*;
use alloy::primitives::{Address, TxHash};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::window;

const STORAGE_KEY: &str = "nexumkit_transactions";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: TxHash,
    pub status: TransactionStatus,
    pub timestamp: u64,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TransactionStore {
    transactions: RwSignal<HashMap<Address, Vec<Transaction>>>,
}

impl TransactionStore {
    pub fn new() -> Self {
        // Load from localStorage if available
        let initial = Self::load_from_storage().unwrap_or_default();

        Self {
            transactions: RwSignal::new(initial),
        }
    }

    /// Add a new transaction to the store
    pub fn add_transaction(&self, address: Address, tx: Transaction) {
        self.transactions.update(|txs| {
            txs.entry(address)
                .or_insert_with(Vec::new)
                .push(tx);
        });

        self.save_to_storage();
    }

    /// Update the status of a transaction
    pub fn update_transaction_status(
        &self,
        address: Address,
        hash: TxHash,
        status: TransactionStatus,
    ) {
        self.transactions.update(|txs| {
            if let Some(address_txs) = txs.get_mut(&address) {
                if let Some(tx) = address_txs.iter_mut().find(|t| t.hash == hash) {
                    tx.status = status;
                }
            }
        });

        self.save_to_storage();
    }

    /// Get all transactions for an address
    pub fn get_transactions(&self, address: Address) -> Vec<Transaction> {
        self.transactions.with(|txs| {
            txs.get(&address)
                .map(|list| list.clone())
                .unwrap_or_default()
        })
    }

    /// Get pending transactions for an address
    pub fn get_pending(&self, address: Address) -> Vec<Transaction> {
        self.transactions.with(|txs| {
            txs.get(&address)
                .map(|list| {
                    list.iter()
                        .filter(|tx| matches!(tx.status, TransactionStatus::Pending))
                        .cloned()
                        .collect()
                })
                .unwrap_or_default()
        })
    }

    /// Get the number of pending transactions for an address
    pub fn pending_count(&self, address: Address) -> usize {
        self.get_pending(address).len()
    }

    /// Clear all transactions for an address
    pub fn clear_transactions(&self, address: Address) {
        self.transactions.update(|txs| {
            txs.remove(&address);
        });

        self.save_to_storage();
    }

    /// Load transactions from localStorage
    fn load_from_storage() -> Option<HashMap<Address, Vec<Transaction>>> {
        let window = window()?;
        let storage = window.local_storage().ok()??;
        let json_str = storage.get_item(STORAGE_KEY).ok()??;

        serde_json::from_str(&json_str).ok()
    }

    /// Save transactions to localStorage
    fn save_to_storage(&self) {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let txs = self.transactions.get();
                if let Ok(json_str) = serde_json::to_string(&txs) {
                    let _ = storage.set_item(STORAGE_KEY, &json_str);
                }
            }
        }
    }
}

/// Provide transaction store in the Leptos context
pub fn provide_transaction_store() -> TransactionStore {
    let store = TransactionStore::new();
    provide_context(store.clone());
    store
}

/// Get transaction store from Leptos context
pub fn use_transaction_store() -> TransactionStore {
    expect_context::<TransactionStore>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_status() {
        let pending = TransactionStatus::Pending;
        let confirmed = TransactionStatus::Confirmed;
        let failed = TransactionStatus::Failed;

        assert_eq!(pending, TransactionStatus::Pending);
        assert_ne!(pending, confirmed);
        assert_ne!(confirmed, failed);
    }
}
