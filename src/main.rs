fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|s| s * s).sum()
}

fn main() {
   
    
}



#[test]
fn returns_expected() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
    
    }