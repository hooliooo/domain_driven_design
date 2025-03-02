use crate::core::domain::category::Category;
use crate::core::domain::tag::Tag;
use chrono::{DateTime, Utc};
use ddd::traits::entity::Entity;
use rust_decimal::Decimal;
use std::borrow::Borrow;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(ddd::Entity, Debug)]
pub struct Transaction {
    #[entity_id]
    id: Uuid,
    #[field]
    purpose: String,
    #[field]
    amount: Decimal,
    #[field]
    date_time: DateTime<Utc>,
    #[field]
    category: Option<Category>,
    #[field]
    tags: HashSet<Tag>,
    #[field]
    child_transactions: HashSet<Transaction>,
}

impl Transaction {
    pub fn new(id: Uuid, purpose: String, amount: Decimal, date_time: DateTime<Utc>) -> Self {
        Self {
            id,
            purpose,
            amount,
            date_time,
            category: None,
            tags: HashSet::default(),
            child_transactions: HashSet::default(),
        }
    }

    pub fn new_now(id: Uuid, purpose: String, amount: Decimal) -> Self {
        Self::new(id, purpose, amount, Utc::now())
    }

    pub fn child_transaction_totals(&self) -> Decimal {
        self.child_transactions
            .iter()
            .fold(Decimal::default(), |mut amount, transaction| {
                amount += transaction.amount();
                amount
            })
    }
}

impl Borrow<Uuid> for Transaction {
    fn borrow(&self) -> &Uuid {
        self.id()
    }
}
