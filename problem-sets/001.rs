// MIT 6.00SC
// Problem Set #001
// Paying Off Credit Card Debt: Paying The Minimum

use std::f64;
use std::io;
use std::process;


// WTF, Rust!  You don't have a built-in function to
// round to specific decimal places?
fn currency(n: f64) -> f64 {
    f64::round(n * 100.0) / 100.0
}

// I guess Rust also overcomplicates user input.
// Was C this obtuse?  I can't quite remember, but
// I think it was.
fn input_number(message: &str) -> f64 {
    print!("{}", message);
    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let balance: f64;

    let trimmed = input_text.trim();
    match trimmed.parse::<f64>() {
        Ok(i) => balance = i,
        Err(..) => {
            println!("Not a valid number: {}", trimmed);
            process::exit(1);
        }
    };

    return balance;
}

fn main() {

    let balance = input_number("Enter the minimum monthly payment rate as a decimal: ");
    let mut new_balance = balance;
    let interest_rate = input_number("Enter the annual credit card interest rate as a decimal: ");
    let monthly_payment_rate = input_number("Enter the minimum monthly payment rate as a \
                                             decimal: ");

    let mut total_amount_paid = 0.0;

    for _ in 1..13 {
        let monthly_payment = currency(new_balance * monthly_payment_rate);
        let interest_paid = currency(interest_rate / 12.0 * new_balance);
        println!("Minimum Monthly: ${:.2}", monthly_payment);
        println!("Interest paid: ${:.2}", interest_paid);
        let principal_paid = monthly_payment - interest_paid;

        new_balance = new_balance - principal_paid;
        total_amount_paid = total_amount_paid + monthly_payment;
    }

    println!("\n\nTotal amount paid: ${:.2}", total_amount_paid);
    println!("Remaining Balance: ${:.2}", new_balance);

}
