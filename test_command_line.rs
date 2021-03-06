use std::io::Write;
use std::str::FromStr;

fn main() {
  let mut numbers = Vec::new();

  for arg in std::env::args().skip(1) {
    numbers.push(u64::from_str(&arg).expect("Error parsing argument"));
  }

  if numbers.len() == 0 {
    writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
    std::process::exit(1);
  }

  let mut d = numbers[0];
}
