use std::fmt;

fn main() {
  let x = 10;
  let y = 20;
  println!("x={0}, y={1}", x, y);

  println!("a={0}, b={1}", a = x, b = y);

  println!(
    "{} of {:b} people know binary, the other half doesn't",
    1, 2
  );

  println!("{number:>width$}", number = 1, width = 5);

  println!("{number:>0width$}", number = 1, width = 5);

  #[allow(dead_code)]
  #[derive(Debug)]
  struct Structure(i32);

  println!("{x:?}", x = Structure(123));

  // Pretty print
  println!("{x:#?}", x = Structure(123));

  let pi = 3.141592;

  println!("Pi is roughly {:.*}", 3, pi);

  diplay_trait();
}

struct Complex(f64, f64);

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Complex {{ real: {0}, imag: {1} }} ",
      self.0,
      self.1
    )
  }
}

fn diplay_trait() {
  println!("{}", Complex(1.1, 2.2) )
}
