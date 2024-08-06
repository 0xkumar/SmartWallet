use std::cell::RefCell;
use candid::{CandidType, Deserialize, Nat};
use ic_cdk_macros::*;
use std::collections::HashMap;

// Define the Account struct
#[derive(CandidType, Deserialize, Clone)]
struct Account {
    owner: String,
    balance: Nat,
}

// Create a thread-local storage for accounts
thread_local! {
    static ACCOUNTS: RefCell<HashMap<String, Account>> = RefCell::new(HashMap::new());
}

// Function to create a new account
#[update]
fn create_account(owner: String) -> bool {
    ACCOUNTS.with(|accounts| {
        if !accounts.borrow().contains_key(&owner) {
            accounts.borrow_mut().insert(owner.clone(), Account {
                owner,
                balance: Nat::from(0),
            });
            true
        } else {
            false
        }
    })
}

// Function to transfer tokens between accounts
#[update]
fn send_tokens(from: String, to: String, amount: Nat) -> bool {
    ACCOUNTS.with(|accounts| {
        let mut accounts = accounts.borrow_mut();
        
        if let (Some(sender), Some(receiver)) = (accounts.get(&from), accounts.get(&to)) {
            if sender.balance >= amount {
                let mut new_sender = sender.clone();
                let mut new_receiver = receiver.clone();
                
                new_sender.balance -= amount.clone();
                new_receiver.balance += amount;
                
                accounts.insert(from, new_sender);
                accounts.insert(to, new_receiver);
                true
            } else {
                false
            }
        } else {
            false
        }
    })
}

// Function to get the balance of an account
#[query]
fn get_balance(owner: String) -> Nat {
    ACCOUNTS.with(|accounts| {
        accounts.borrow().get(&owner).map_or(Nat::from(0), |account| account.balance.clone())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        let owner = "Alice".to_string();
        assert!(create_account(owner.clone()));
        assert_eq!(get_balance(owner), Nat::from(0));
    }

    #[test]
    fn test_send_tokens() {
        let alice = "Alice".to_string();
        let bob = "Bob".to_string();
        create_account(alice.clone());
        create_account(bob.clone());

        // Simulate receiving tokens for Alice
        ACCOUNTS.with(|accounts| {
            let mut accounts = accounts.borrow_mut();
            if let Some(account) = accounts.get_mut(&alice) {
                account.balance = Nat::from(100);
            }
        });

        assert!(send_tokens(alice.clone(), bob.clone(), Nat::from(50)));
        assert_eq!(get_balance(alice), Nat::from(50));
        assert_eq!(get_balance(bob), Nat::from(50));
    }
}