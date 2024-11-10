use Budget_Tracker_Rust::transaction::{add_transaction, Transaction};

#[test]
fn test_add_transaction_in_test_mode() {
    std::env::set_var("TEST_MODE", "true");

    let transaction = add_transaction();

    assert_eq!(transaction.date, "2024-11-10");
    assert_eq!(transaction.description, "Groceries");
    assert_eq!(transaction.amount, 50.0);
    assert_eq!(transaction.category, "Food");

    std::env::remove_var("TEST_MODE");
}
