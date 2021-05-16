use std::fmt;

struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

impl fmt::Display for Satellite {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Satellite Name: {}, Velocity: {}", self.name, self.velocity)
  }
}

/* YOUR CODE GOES HERE */

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("hubble is {}", hubble);
}