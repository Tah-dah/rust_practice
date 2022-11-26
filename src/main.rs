fn html_special_chars(html: &str) -> String {
    let mut result = String::new();
    for character in html.chars() {
      match character {
        '<' => result.push_str("&lt;"),
        '>' => result.push_str("&gt;"),
        '"' => result.push_str("&quot;"),
        '&' => result.push_str("&amp;"),
        _ => result.push(character)
      }
    }
    result
  }                                                                                                                               
    
fn main() {
   
    
}

//use super::html_special_chars;

#[test]
fn sample_tests() {
    assert_eq!(html_special_chars("<h2>Hello World</h2>"), 
        "&lt;h2&gt;Hello World&lt;/h2&gt;");
    assert_eq!(html_special_chars("Hello, how would you & I fare?"), 
        "Hello, how would you &amp; I fare?");
    assert_eq!(html_special_chars("How was \"The Matrix\"?  Did you like it?"), 
        "How was &quot;The Matrix&quot;?  Did you like it?");
    assert_eq!(html_special_chars("<script>alert('Website Hacked!');</script>"), 
        "&lt;script&gt;alert('Website Hacked!');&lt;/script&gt;");
}