use crate::core::domain::transaction::Transaction;
use ddd::traits::entity::Entity;
use iso_currency::Currency;
use rust_decimal::Decimal;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(ddd::Aggregate, Debug)]
pub struct Account {
    #[generate_id(Uuid)]
    #[entity_id]
    id: AccountId,
    #[field]
    name: String,
    #[field]
    amount: Decimal,
    #[field]
    currency: Currency,
    #[field]
    transactions: HashSet<Transaction>,
}

impl Account {
    pub fn new(id: Uuid, name: String, amount: Decimal, currency: Currency) -> Self {
        Self {
            id: AccountId::new(id),
            name,
            amount,
            currency,
            transactions: HashSet::default(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.insert(transaction);
    }

    pub fn remove_transaction(&mut self, transaction: Transaction) {
        self.transactions.remove(&transaction);
    }

    pub fn total(&self) -> Decimal {
        self.transactions
            .iter()
            .fold(Decimal::default(), |mut curr, element| {
                curr += element.amount();
                curr
            })
    }

    pub fn transactions_sorted_by_date_ascending(&self) -> Vec<&Transaction> {
        let transactions = {
            let mut transactions = self.transactions.iter().collect::<Vec<&Transaction>>();
            transactions.sort_by_key(|transaction| transaction.date_time());
            transactions
        };
        transactions
    }

    pub fn transactions_sorted_by_date_descending(&self) -> Vec<&Transaction> {
        let transactions = {
            let mut transactions = self.transactions.iter().collect::<Vec<&Transaction>>();
            transactions.sort_by(|left, right| right.date_time().cmp(left.date_time()));
            transactions
        };
        transactions
    }
}
