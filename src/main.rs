fn combat(health: f32, damage: f32) -> f32 {
    let result = if health - damage > 0.0 { health - damage } else { 0.0 };
    return result
}

fn main() {
    println!("{:?}",combat(100.0, 5.0));
    
    println!("{:?}",combat(20.0, 30.0));
    
}