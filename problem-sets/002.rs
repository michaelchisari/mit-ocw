// MIT 6.00SC
// Problem Set #002
// Paying Off Credit Card Debt: Paying Debt Off In A Year

use std::f64;
use std::io;
use std::process;


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

// Another WTF rust.  Loops are privately scoped, but also don't let you
// return a value.  Apparently this is a pain point for a lot of people.
// https://github.com/rust-lang/rfcs/issues/961
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
    let mut final_balance = balance;
    let interest_rate = input_number("Enter the annual credit card interest rate as a decimal: ");

    let monthly_interest_rate = interest_rate / 12.0;

    let mut step = 10.0;

    let mut final_months = 0;

    'outer: while step < balance {
        let (month_count, new_balance) = monthly_loop(balance, monthly_interest_rate, step);
        if month_count > 0 {
            final_months = month_count;
            final_balance = new_balance;
            break;
        }

        step = step + 10.0;
    }

    println!("Monthly payment to pay off debt in 1 year: ${:.2}", step);
    println!("Number of months needed: {:.2}", final_months);
    println!("Balance: ${:.2}", final_balance);
}
