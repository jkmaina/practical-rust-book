// Loan Eligibility Checker
// This program checks if a user is eligible for different types of loans based on their financial information
// such as income, credit score, employment status, and debt-to-income ratio.
// It provides detailed feedback on eligibility and reasons for ineligibility.
// The program uses logical operators to combine conditions for eligibility checks.
// It also includes helper functions for input validation.
// The program is designed to be user-friendly, guiding the user through the input process and providing
// clear feedback on their loan eligibility status.
// The eligibility criteria are:
// - Premium Loan: Good credit score (700+), employed, very low DTI (
//   debt-to-income ratio <= 0.28)
// - Standard Loan: Acceptable credit score (600+), employed, low DTI (
//   debt-to-income ratio <= 0.36)
// - Basic Loan: Credit score >= 500, either employed or income > 50,000, and DTI <= 0.45
// If the user does not qualify for any loans, the program provides specific reasons for ineligibility.
// The program uses Rust's standard input/output library for user interaction.

use std::io;
fn main() {
    println!("Loan Eligibility Checker");
    
    // Get income
    println!("What is your annual income?");
    let income = get_positive_number();
    
    // Get credit score
    println!("What is your credit score? (300-850)");
    let credit_score = get_number_in_range(300, 850);
    
    // Get employment status
    println!("Are you currently employed? (yes/no)");
    let is_employed = get_yes_no_input();
    
    // Get existing debt
    println!("What is your current total debt?");
    let debt = get_positive_number();
    
    // Calculate debt-to-income ratio
    let dti_ratio = debt / income;
    
    // Determine eligibility
    let has_good_credit = credit_score >= 700;
    let has_acceptable_credit = credit_score >= 600;
    let has_low_dti = dti_ratio <= 0.36;
    let has_very_low_dti = dti_ratio <= 0.28;
    
    // Loan eligibility rules
    let is_eligible_for_premium = 
        has_good_credit && 
        is_employed && 
        has_very_low_dti;
    
    let is_eligible_for_standard = 
        has_acceptable_credit && 
        is_employed && 
        has_low_dti;
    
    let is_eligible_for_basic = 
        credit_score >= 500 && 
        (is_employed || income > 50000.0) && 
        dti_ratio <= 0.45;
    
    // Display results
    println!("\nLoan Eligibility Results:");
    println!("------------------------");
    
    if is_eligible_for_premium {
        println!("✓ Eligible for Premium Loan (lowest interest rate)");
    } else {
        println!("✗ Not eligible for Premium Loan");
    }
    
    if is_eligible_for_standard {
        println!("✓ Eligible for Standard Loan");
    } else {
        println!("✗ Not eligible for Standard Loan");
    }
    
    if is_eligible_for_basic {
        println!("✓ Eligible for Basic Loan (higher interest rate)");
    } else {
        println!("✗ Not eligible for Basic Loan");
    }
    
    if !is_eligible_for_premium && !is_eligible_for_standard && !is_eligible_for_basic {
        println!("\nSorry, you are not eligible for any loans at this time.");
        
        // Provide specific reasons
        if !has_acceptable_credit {
            println!("Reason: Your credit score is too low.");
        }
        
        if !is_employed && income <= 50000.0 {
            println!("Reason: You need to be employed or have higher income.");
        }
        
        if dti_ratio > 0.45 {
            println!("Reason: Your debt-to-income ratio is too high.");
            println!("Your current DTI: {:.2}", dti_ratio);
            println!("Maximum allowed: 0.45");
        }
    }
}
// Helper function to get a positive number
fn get_positive_number() -> f64 {
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse() {
            Ok(num) if num > 0.0 => return num,
            _ => println!("Please enter a positive number:"),
        }
    }
}
// Helper function to get a number within a specific range
fn get_number_in_range(min: u32, max: u32) -> u32 {
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse() {
            Ok(num) if num >= min && num <= max => return num,
            _ => println!("Please enter a number between {} and {}:", min, max),
        }
    }
}
// Helper function to get yes/no input
fn get_yes_no_input() -> bool {
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please enter 'yes' or 'no':"),
        }
    }
}