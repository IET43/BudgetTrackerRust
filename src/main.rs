pub mod transaction;

use transaction::add_transaction;

fn main() {
    let transaction = add_transaction();
    println!("Transaction added: {:?}", transaction);
}
