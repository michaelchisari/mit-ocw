use std::f64;


// WTF, Rust!  You don't have a built-in function to
// round to specific decimal places?
fn currency(n: f64) -> f64 {
    f64::round(n * 100.0) / 100.0
}

fn main() {

    // Say you’ve made a $5,000 purchase on a credit card
    // with 18% annual interest rate
    // and 2% minimum monthly payment rate.
    // After a year, how much is the remaining balance?

    let balance = 5000.00;
    let mut new_balance = balance;
    let interest_rate = 0.18;
    let monthly_payment_rate = 0.02;

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
