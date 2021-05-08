fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let max: i32;
    let min: i32;
    let mean: f64;

    let mut temp_max = 0;
    let mut temp_min = 0;
    let mut sum = 0;

    for (index, num) in numbers.iter().enumerate() {
        println!("num is {}", num);
        temp_max = if index == 0  {*num} else if num > &temp_max {*num} else {temp_max};
        temp_min = if index == 0 {*num} else if num < &temp_min {*num} else {temp_min};
        sum = sum + num;
    }
    max = temp_max;
    min = temp_min;
    mean = sum as f64 / numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}