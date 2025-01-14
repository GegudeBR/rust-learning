use std::io::{self, Write};

fn fibonacci(n: usize, cache: &mut Vec<u128>) -> u128 {
  if let Some(&result) = cache.get(n) {
    result
  } else {
    let result = fibonacci(n - 1, cache) + fibonacci(n - 2, cache);
    cache.resize(n+1, result);
    result
  }
}

fn main() {
  let mut cache: Vec<u128> = vec![]; 
  cache.push(0);
  cache.push(1);

  loop {
    let number = loop {
    print!("Fibonacci of n = ");
    let _ = io::stdout().flush();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    if let Ok(val) = input_line.trim().parse::<usize>() {
      break val;
    }
  };

  println!("F{} = {}", number, fibonacci(number, &mut cache))
  }
}