
    // [x] create rectangle struct: width height f64
    // [x] create get_area() w * h
    // [x] create scale() * w and h by input f64
    // [x] create new()  input w, h -> new Rectangle


struct Rectangle {
  width: f64,
  height: f64
}

impl Rectangle {
  fn new(width: f64, height: f64) -> Rectangle {
    Rectangle {
      width: width,
      height: height
    }
  }

  fn get_area(&self) -> f64 {
    &self.width * &self.height
  }

  fn scale(&mut self, scale: f64) {
    self.width = self.width * scale;
    self.height = self.height * scale;
  }

}

fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}