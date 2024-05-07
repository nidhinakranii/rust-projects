fn main() {
    let mut account_balance = 1000;
    // Example of while loop
    let mut months = 1;
    let mut interest = 0.0;

    println!("Your account balance: ${}", account_balance);

    while months <= 12 {
        let interest_rate = 0.05; // 5% annual interest rate
        interest = (account_balance as f32 * interest_rate) / 12.0;
        account_balance += interest as i32;
        months += 1;
    }
    println!(
        "After a year, your account balance with interest: ${}",
        account_balance
    );

    // Example of for loop
    for month in 1..=12 {
        println!(
            "Transaction for month {}: Interest earned ${}",
            month, interest
        );
    }
}
