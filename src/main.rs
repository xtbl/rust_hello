fn main() {
    // let x: u8 = 255;
    let x = 10.2;
    println!("Hello world!");
    println!("x is {}", x);

    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3 as f64;

    assert_eq!(average, 45.1);
}
