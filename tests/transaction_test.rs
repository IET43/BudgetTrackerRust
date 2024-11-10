use Budget_Tracker_Rust::transaction::{create_transaction, Transaction};

#[test]
fn test_create_transaction() {
    let date = "2024-11-10".to_string();
    let description = "Groceries".to_string();
    let amount = 50.0;
    let category = "Food".to_string();

    let transaction = create_transaction(date.clone(), description.clone(), amount, category.clone());

    assert_eq!(transaction.date, date);
    assert_eq!(transaction.description, description);
    assert_eq!(transaction.amount, amount);
    assert_eq!(transaction.category, category);
}
