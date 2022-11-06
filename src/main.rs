//function takes two integers and returns an array of integeres between input parameters

fn between(a: i16, b: i16) -> Vec<i16> {
    (a..=b).collect()
    //Vec::from_iter (a..=b);
}

fn main() {
   
}