fn greet() -> &'static str {
    "hello world!"
}

fn main() {
   
    
}



#[test]
fn test_greets_the_world() {
    assert_eq!(greet(), "hello world!", "should return the correct message");
}