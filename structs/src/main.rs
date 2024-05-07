// Define a struct named BankAccount
#[derive(Debug)] // Add this derive attribute to implement Debug trait
struct BankAccount {
    owner_name: String,
    balance: i32,
    interest_rate: f32,
    custom_field: String,
}

fn main() {
    let user1 = BankAccount {
        owner_name: "Nidhi Nakrani".to_string(),
        balance: 1000,
        interest_rate: 0.05,
        custom_field: "Custom Data".to_string(),
    };

    // Print the values of the user1 BankAccount instance
    println!("Owner Name: {}", user1.owner_name);
    println!("Balance: ${}", user1.balance);
    println!("Interest Rate: {}%", user1.interest_rate);
    println!("Custom Field: {}", user1.custom_field);

    // Now, you can print the user1 struct using {:?}
    println!("user1 is {:?}", user1);
}
