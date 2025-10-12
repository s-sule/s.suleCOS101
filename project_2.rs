fn main() {
    // Define sales data
    let items = ["Toshiba", "Mac", "HP", "Dell", "Acer"];
    let qty = [2, 1, 3, 4, 1];
    let amount = [450_000.0, 1_500_000.0, 750_000.0, 2_850_000.0, 250_000.0];

    // Calculate total and average
    let total_sales: f64 = amount.iter().sum();
    let average_sales = total_sales / amount.len() as f64;

    // Display the sales records
    println!("---------------------------------------------");
    println!("S/N\tItem\t\tQty\tAmount");
    println!("---------------------------------------------");

    for i in 0..items.len() {
        println!(
            "{}\t{}\t\t{}\t₦{:.2}",
            i + 1,
            items[i],
            qty[i],
            amount[i]
        );
    }

    println!("---------------------------------------------");
    println!("Total Sales: ₦{:.2}", total_sales);
    println!("Average Sales: ₦{:.2}", average_sales);
}
