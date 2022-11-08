// Define a function "square" here which takes a signed integer of type i32
// and returns the square of that integer
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    println!("{:?}", square(1));
    println!("{:?}", square(5));
    
}