enum Location {
  Unknown,
  Anonymous,
  Known(f64, f64)
}

trait Display {
  fn display(&self) -> String {
    String::from("default display")
  }
}

impl Display for Location {
  fn display(&self) -> String {
    match self {
      Location::Unknown => println!("location for unknown"),
      Location::Anonymous => println!("location for anon"),
      Location::Known(x,y) => println!("location for known {}, {}", x, y),
    }
    String::from("default display")
  }
}

fn main() {
  let address = Location::Unknown;
  address.display();
  let address = Location::Anonymous;
  address.display();
  let address = Location::Known(28.608295, -80.604177);
  address.display();
}
