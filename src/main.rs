fn abbrev_name(name: &str) -> String {
    let mut names = name.split(" ");
    let first = names.next().unwrap();
    let second = names.next().unwrap();
    return first[0..1].to_uppercase().to_string() + "." + &second[0..1].to_uppercase();
}
fn main() {
   
    
}



#[test]
fn sample_tests() {
    assert_eq!(abbrev_name("Sam Harris"), "S.H");
    assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
    assert_eq!(abbrev_name("Evan Cole"), "E.C");
    assert_eq!(abbrev_name("P Favuzzi"), "P.F");
    assert_eq!(abbrev_name("David Mendieta"), "D.M");
  }