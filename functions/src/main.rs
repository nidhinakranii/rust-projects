fn main() {
    // Initialize a bank account
    let mut account_balance = 1000;

    // Display initial balance
    println!("Your account balance: ${}", account_balance);

    // Deposit some money
    account_balance = deposit(500, account_balance);

    // Withdraw some money
    account_balance = withdraw(200, account_balance);

    // Display final balance
    println!("Your final account balance: ${}", account_balance);
}

// Function to deposit money into the account
fn deposit(amount: i32, mut balance: i32) -> i32 {
    balance += amount;
    println!("Deposited ${}", amount);
    balance // Return the updated balance
}

// Function to withdraw money from the account
fn withdraw(amount: i32, mut balance: i32) -> i32 {
    if amount <= balance {
        balance -= amount;
        println!("Withdrawn ${}", amount);
    } else {
        println!("Insufficient funds!");
    }
    balance // Return the updated balance
}
