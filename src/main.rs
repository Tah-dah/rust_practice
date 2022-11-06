fn invert(mut values: Vec<i32>) -> Vec<i32> {
    values.iter().map(|x| x * -1).collect()
    
}

fn main() {
   let  vec = vec![1,2,3,4,5,6,7,8,9];
   println!("{:?}", invert(vec));
}