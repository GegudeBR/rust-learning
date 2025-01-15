use std::io::{self};

fn triangle(num: u32) -> String {
  let mut buffer = String::with_capacity(num as usize);
  for _ in 0..num {
    buffer.push('*');
  }

  buffer
}

fn main() {

  let number = loop {
    println!("Enter a number: ");

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    if let Ok(val) = input_line.trim().parse::<u32>() {
      break val;
    }
  };

  let mut i = 0;
  while i <= number {
    println!("{}", triangle(i));
    i += 1;
  }

}