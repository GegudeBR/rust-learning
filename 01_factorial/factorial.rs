use std::io::{self, Write};

fn factorial (num: i32) -> i32 {

  match num {
    x if x <= 1 => return 1,
    x => x * factorial(x-1),
  }

}

fn main() {

  let mut input_line = String::new();

  println!("Factorial Calulator");
  print!("n = ");
  let _ = io::stdout().flush();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");

  let x: i32 = input_line.trim().parse().expect("Input not a number");

  println!("n! = {}", factorial(x));
}