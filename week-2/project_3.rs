fn main() {
    // Given values
    let principal: f64 = 510000.0; // Initial cost of TV
    let rate: f64 = 5.0;           // Depreciation rate per annum
    let years: f64 = 3.0;          // Number of years

    // Calculate depreciation
    let amount = principal * (1.0 - (rate / 100.0)).powf(years);

    // Output results
    println!("--- Depreciation Calculation ---");
    println!("Initial Value (P): ₦{:.2}", principal);
    println!("Rate of Depreciation (R): {:.2}%", rate);
    println!("Time (n): {:.0} years", years);
    println!("Value after depreciation (A): ₦{:.2}", amount);
}