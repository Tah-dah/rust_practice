fn quarter_of(month: u8) -> u8 {
    match month {
        1..=3 => 1,
        4..=6 => 2,
        7..=9 => 3,
        10..=12 => 4,
        _=> 0
        
    }
}

fn main() {
    println!("{:?}",quarter_of(3));
    println!("{:?}",quarter_of(12));
    println!("{:?}",quarter_of(5));
    println!("{:?}",quarter_of(6));
}