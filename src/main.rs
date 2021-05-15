use std::ops::Add;

fn sum_boxes<T: Add>(boxed_a: Box<T>, boxed_b: Box<T>) -> Box<<T as Add>::Output> {

  let unboxed_a = *boxed_a;
  let unboxed_b = *boxed_b;
  let sum = unboxed_a + unboxed_b;

  let boxed_sum = Box::new(sum);
  boxed_sum
}

fn main() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);

    println!("Tests passed!");
}