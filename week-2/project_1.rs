fn main() {
    // Given values
    let principal: f64 = 520_000_000.0; // ₦520,000,000
    let rate: f64 = 10.0;               // 10% per annum
    let years: f64 = 5.0;               // 5 years

    // Formula for Compound Interest
    let amount = principal * (1.0 + rate / 100.0).powf(years);
    let compound_interest = amount - principal;

    println!("PROJECT I - Compound Interest Calculation");
    println!("----------------------------------------");
    println!("Principal (P): ₦{:.2}", principal);
    println!("Rate (R): {:.2}%", rate);
    println!("Time (n): {:.0} years", years);
    println!("----------------------------------------");
    println!("Amount (A): ₦{:.2}", amount);
    println!("Compound Interest (CI): ₦{:.2}", compound_interest);
}