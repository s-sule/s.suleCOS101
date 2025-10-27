use std::io;

fn main() {
    let mut input = String::new();

    // Input experience
    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut input).unwrap();
    let experience = input.trim().to_lowercase();
    input.clear();

    // Input age
    println!("Enter the age of the employee:");
    io::stdin().read_line(&mut input).unwrap();
    let age: u32 = input.trim().parse().unwrap();

    // Determine incentive
    let incentive: u32;

    if experience == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age < 40 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            incentive = 1_000_000; // Default for experienced between 28-29 if needed
        }
    } else {
        incentive = 100_000;
    }

    println!("The incentive for this employee is ₦{}.", incentive);
}
