
fn usdcny(usd: u16) -> String {

    (f64::from(usd) * 6.75).to_string() + " Chinese Yuan"
}

fn main() {
    println!("{:?}", usdcny(15));
    println!("{:?}", usdcny(465));
    
}