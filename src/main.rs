
fn arr(n: usize) -> Vec<u32> {
    let mut arr:Vec<u32> = vec![];
    for s in 0..n as u32 {
        arr.push(s)
    }
    arr
}
fn main() {
   
    
}

#[test]
    fn example_tests() {
        assert_eq!(arr(0), vec![]);
        assert_eq!(arr(4), vec![0,1,2,3]);
    }
