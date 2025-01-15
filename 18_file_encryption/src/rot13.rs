pub fn rot13(line: &String) -> String {
  let mut encrypted = String::new();
  for c in line.chars() {
    if c.is_alphabetic() {
      let base = if c.is_lowercase() { 'a' } else { 'A' };
      encrypted.push(((c as u8 - base as u8 + 13) % 26 + base as u8) as char);
    } else {
      encrypted.push(c);
    }
  }
  encrypted
}

#[test]
fn converts() {
  let input = "Hello World!".to_string();
  let converted = "Uryyb Jbeyq!".to_string();
  assert!(rot13(&input) == "Uryyb Jbeyq!");
  assert!(rot13(&converted) == "Hello World!");
}