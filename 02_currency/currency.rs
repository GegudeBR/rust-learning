use std::io::{self};

const USD_TO_BRL: f32 = 6.09570;
const BRL_TO_USD: f32 = 0.16405003;

fn convert(num: f32, convert_to:&str) -> f32 {
  match convert_to {
    "USD" => return num * BRL_TO_USD,
    "BRL" => return num * USD_TO_BRL,
    _ => panic!(),
  }
}

fn main() {


  let option = loop {
    println!("Choose from the options below: ");
    println!("(1) BRL to USD");
    println!("(2) USD to BRL");

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    if let Ok(val) = input_line.trim().parse::<u8>() {
      if val >= 1 && val <= 2 {
        break val;
      }
    }
  };

  match option {
    1 => {
      let brl = loop {
        println!("Enter amount in $");
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        if let Ok(val) = input_line.trim().parse::<f32>() {
          break val;
        }
      };

      println!("$ {:.2}", convert(brl, "USD"));
    },
    2 => {
      let usd = loop {
        println!("Enter amount in $");
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        if let Ok(val) = input_line.trim().parse::<f32>() {
          break val;
        }
      };
      println!("R$ {:.2}", convert(usd, "BRL"));
    },
    _ => panic!(),
  }
  
}