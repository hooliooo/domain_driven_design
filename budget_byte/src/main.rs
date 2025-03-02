use crate::core::domain::account::Account;
use crate::core::domain::transaction::Transaction;
use chrono::Utc;
use iso_currency::Currency;
use rust_decimal::Decimal;
use uuid::Uuid;

pub mod core;

fn main() {
    let transaction_one = Transaction::new(
        Uuid::new_v4(),
        String::from("Cash"),
        Decimal::new(100, 0),
        Utc::now(),
    );

    let transaction_two = Transaction::new(
        Uuid::new_v4(),
        String::from("Cash"),
        Decimal::new(10_000, 3),
        Utc::now(),
    );

    let mut account = Account::new(
        Uuid::new_v4(),
        String::from("Cash"),
        Decimal::new(0, 0),
        Currency::EUR,
    );

    account.add_transaction(transaction_one);
    account.add_transaction(transaction_two);

    println!("Total: {:?}", account.total());
    println!("Account: {:?}", account)
}
