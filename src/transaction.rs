use std::io;

#[derive(Debug)]
pub struct Transaction {
    pub date: String,
    pub description: String,
    pub amount: f64,
    pub category: String,
}

pub fn get_user_input(prompt: &str) -> String {
    if std::env::var("TEST_MODE").is_ok() {
        match prompt {
            "Enter the date (YYYY-MM-DD):" => "2024-11-10".to_string(),
            "Enter the description:" => "Groceries".to_string(),
            "Enter the amount:" => "50.0".to_string(),
            "Enter the category:" => "Food".to_string(),
            _ => "".to_string(),
        }
    } else {
        println!("{}", prompt);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }
}

pub fn create_transaction(date: String, description: String, amount: f64, category: String) -> Transaction {
    Transaction {
        date,
        description,
        amount,
        category,
    }
}

pub fn add_transaction() -> Transaction {
    let date = get_user_input("Enter the date (YYYY-MM-DD):");
    let description = get_user_input("Enter the description:");
    let amount: f64 = get_user_input("Enter the amount:")
        .parse()
        .expect("Please enter a valid number");
    let category = get_user_input("Enter the category:");

    create_transaction(date, description, amount, category)
}

