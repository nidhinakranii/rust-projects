fn main() {
    // Initialize a bank account
    let mut account_balance = 1000;

    // Display initial balance
    println!("Your account balance: ${}", account_balance);

    // Deposit some money
    deposit(500, &mut account_balance);

    // Withdraw some money
    withdraw(200, &mut account_balance);

    // Display final balance
    println!("Your final account balance: ${}", account_balance);
}

// Function to deposit money into the account
fn deposit(amount: i32, balance: &mut i32) {
    *balance += amount;
    println!("Deposited ${}", amount);
}

// Function to withdraw money from the account
fn withdraw(amount: i32, balance: &mut i32) {
    if amount <= *balance {
        *balance -= amount;
        println!("Withdrawn ${}", amount);
    } else {
        println!("Insufficient funds!");
    }
}
