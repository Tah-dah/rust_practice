fn no_space(x : String) -> String{
    x.replace(" ", "").trim().to_string()
}

fn main() {
    let string_x = "8 j 8   mBliB8g  imjB8B8  jl  B";
    println!("{}", no_space(string_x.to_string()));

}