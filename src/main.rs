fn count_sheep(sheep: &[bool]) -> u8 {
    sheep              // take the sheep array
      .iter()          // turn it into an iterable
      .filter(|&&x| x) // filter it by taking the values in the array and returning only the true ones
      .count() as u8   // count all of the elements in the filtered array and return a u8
  }
fn main() {
   
    
}



#[test]
fn returns_correct_sheep_count() {
    assert_eq!(count_sheep(&[false]), 0);
    assert_eq!(count_sheep(&[true]), 1);
    assert_eq!(count_sheep(&[true, false]), 1);
  }