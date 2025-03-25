use rust_decimal::prelude::*;
fn main() {
    let mut bd = Decimal::new(2500, 0);
    println!("Original: {}", bd);

    bd.rescale(18);
    println!("Rescale 18: {}", bd);
    
    let pow18 = Decimal::from_scientific("10e18").unwrap();
    bd *= pow18;
    println!("Result 18 decimals: {}", bd);
}
