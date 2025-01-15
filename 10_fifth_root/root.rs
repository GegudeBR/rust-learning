fn fifth_root(num: f32) -> f32 {
  return num.powf(1.0/5.0);
}

fn main() {
  let mut sum = 0;
  for x in 0..100 {
    if x % 2 == 0 {
      sum += x * x;
    }
  }

  println!("Fifth root of {}, is: {}", sum , fifth_root(sum as f32));
}