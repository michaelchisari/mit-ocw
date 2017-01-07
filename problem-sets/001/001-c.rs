// MIT 6.00SC
// Problem Set #001-c
// Paying Off Credit Card Debt: Paying Debt Off In A Year (Bisection Search)

use std::f64;
use std::io;
use std::process;


// Round to two decimal places
fn currency(n: f64) -> f64 {
    f64::round(n * 100.0) / 100.0
}

// Get number input from stdio
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

// Loop through the year, checking whether the step value works
fn monthly_loop(balance: f64, monthly_interest_rate: f64, step: f64) -> (i32, f64) {
    let mut new_balance = balance;
    for n in 1..13 {
        new_balance = new_balance * (1.0 + monthly_interest_rate) - step;
        if new_balance < 0.0 {
            return (n, new_balance);
        }
    }

    return (0, 0.0);
}

fn main() {

    let balance = input_number("Enter the outstanding balance on your credit card: ");
    let final_balance: f64;
    let interest_rate = input_number("Enter the annual credit card interest rate as a decimal: ");

    let monthly_interest_rate = interest_rate / 12.0;

    let mut lower_bound = balance / 12.0;
    let mut upper_bound = (balance * (1.0 + monthly_interest_rate).powf(12.0)) / 12.0;

    let final_months: i32;

    println!("Lower: {}", lower_bound);
    println!("Upper: {}", upper_bound);

    let mut step: f64;

    'outer: loop {
        step = (upper_bound + lower_bound) / 2.0;
        let (month_count, new_balance) = monthly_loop(balance, monthly_interest_rate, step);
        if month_count > 0 {
            if new_balance > -1.0 {
                final_months = month_count;
                final_balance = new_balance;
                break;
            } else {
                // Amount was too big.  Move the upper bound down.
                upper_bound = currency((upper_bound + lower_bound) / 2.0);
            }
        } else {
            // Amount was too small.  Move the lower bound up.
            lower_bound = currency((upper_bound + lower_bound) / 2.0);
        }
    }

    println!("Monthly payment to pay off debt in 1 year: ${:.2}", step);
    println!("Number of months needed: {:.2}", final_months);
    println!("Balance: ${:.2}", final_balance);
}
