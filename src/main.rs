//match numberof peddels returns phrase 

fn how_much_i_love_you(nb_petals: u16) -> &'static str {
    
    match nb_petals % 6u16 {
        1 => "I love you",
        2 => "a little",
        3 => "a lot",
        4 => "passionately",
        5 => "madly",
        _ => "not at all",
    }
}

fn main() {
   
}