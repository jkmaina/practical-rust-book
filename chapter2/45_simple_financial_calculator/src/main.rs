// Simple Financial Calculator in Rust
// This program calculates monthly payments, total cost, and interest for a loan,
// and provides insights on affordability and extra payments.

fn main() {
    // Loan parameters
    let principal: f64 = 100_000.0;  // Loan amount in dollars
    let annual_rate: f64 = 0.05;     // 5% annual interest rate
    let loan_term_years: u32 = 30;   // 30-year loan
    
    // Convert annual rate to monthly rate
    let monthly_rate: f64 = annual_rate / 12.0;
    
    // Convert loan term to months
    let loan_term_months: u32 = loan_term_years * 12;
    
    // Calculate monthly payment using the loan formula
    // M = P * (r * (1 + r)^n) / ((1 + r)^n - 1)
    // Where:
    // M = monthly payment
    // P = principal (loan amount)
    // r = monthly interest rate
    // n = number of payments (loan term in months)
    
    let term_factor: f64 = (1.0 + monthly_rate).powi(loan_term_months as i32);
    let monthly_payment: f64 = principal * monthly_rate * term_factor / (term_factor - 1.0);
    
    // Calculate total cost
    let total_cost: f64 = monthly_payment * loan_term_months as f64;
    let total_interest: f64 = total_cost - principal;
    
    // Display results
    println!("Loan Amount: ${:.2}", principal);
    println!("Annual Interest Rate: {:.2}%", annual_rate * 100.0);
    println!("Loan Term: {} years", loan_term_years);
    println!("Monthly Payment: ${:.2}", monthly_payment);
    println!("Total Cost: ${:.2}", total_cost);
    println!("Total Interest: ${:.2}", total_interest);
    
    // Calculate how much goes to principal vs. interest in the first payment
    let first_month_interest: f64 = principal * monthly_rate;
    let first_month_principal: f64 = monthly_payment - first_month_interest;
    
    println!("\nFirst Month Payment Breakdown:");
    println!("Principal: ${:.2}", first_month_principal);
    println!("Interest: ${:.2}", first_month_interest);
    
    // Determine if this is an affordable loan (example threshold)
    const AFFORDABLE_THRESHOLD: f64 = 1_500.0;
    let is_affordable = monthly_payment < AFFORDABLE_THRESHOLD;
    
    println!("\nIs this loan affordable? {}", if is_affordable { "Yes" } else { "No" });
    
    // Calculate how much you'd save by paying an extra $100 per month
    let extra_payment: f64 = 100.0;
    let mut remaining_balance: f64 = principal;
    let mut months_to_payoff: u32 = 0;
    
    while remaining_balance > 0.0 {
        let interest_payment = remaining_balance * monthly_rate;
        let principal_payment = monthly_payment + extra_payment - interest_payment;
        
        remaining_balance -= principal_payment;
        months_to_payoff += 1;
        
        // Ensure we don't go below zero due to floating-point precision
        if remaining_balance < 0.0 {
            remaining_balance = 0.0;
        }
        
        // Safety check to prevent infinite loops
        if months_to_payoff > loan_term_months * 2 {
            println!("Calculation error: too many iterations");
            break;
        }
    }
    
    let years_to_payoff: f64 = months_to_payoff as f64 / 12.0;
    let years_saved: f64 = loan_term_years as f64 - years_to_payoff;
    
    println!("\nWith an extra ${:.2} monthly payment:", extra_payment);
    println!("Payoff time: {:.1} years", years_to_payoff);
    println!("Years saved: {:.1}", years_saved);
}
