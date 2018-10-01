use std::fmt;

struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let red = self.red;
    let green = self.green;
    let blue = self.blue;
    writeln!(
      f,
      "RGB ({red}, {green}, {blue}) 0x{red:02X}{green:02X}{blue:02X}",
      red = red,
      green = green,
      blue = blue
    )
  }
}

fn main() {
  for color in [
    Color {
      red: 128,
      green: 255,
      blue: 90,
    },
    Color {
      red: 0,
      green: 3,
      blue: 254,
    },
    Color {
      red: 0,
      green: 0,
      blue: 0,
    },
  ].iter()
  {
    println!("{}", color);
  }
}
