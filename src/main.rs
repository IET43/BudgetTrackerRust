use std::io;

#[derive(Debug)]
struct Transaction {
    date: String,
    description: String,
    amount: f64,
    category: String,
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn add_transaction() -> Transaction {
    let date = get_user_input("Enter the date (YYYY-MM-DD):");
    let description = get_user_input("Enter the description:");
    let amount: f64 = get_user_input("Enter the amount:")
        .parse()
        .expect("Please enter a valid number");
    let category = get_user_input("Enter the category:");

    Transaction {
        date,
        description,
        amount,
        category,
    }
}

fn main() {
    let transaction = add_transaction();
    println!("Transaction added: {:?}", transaction);
}
