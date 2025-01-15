use std::io;

fn main() {
  println!("Type a string:");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();

  let vowels = vec!['a', 'e', 'i', 'o', 'u'];
  let mut vowel_count = 0;
  let mut consonant_count = 0;

  for c in input_line.chars() {
    if !c.is_alphabetic() {
      break;
    }

    if vowels.contains(&c) {
      vowel_count += 1;
    } else {
      consonant_count += 1;
    }
  }

  println!("Vowels: {}", vowel_count);
  println!("Consonants: {}", consonant_count)
}