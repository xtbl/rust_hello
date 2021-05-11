fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

fn trim_spaces(s: &str) -> &str {
    // use as slice byte
    let bytes = s.as_bytes();

    let mut first_index: usize = 1000000;
    let mut last_index: usize = 1000000;
    for (index, &item) in bytes.iter().enumerate() {
        if item != b' ' {
            first_index = index;
            break;
        }
    }
    for (index2, &item2) in bytes.iter().enumerate().rev() {
        if item2 != b' ' {
            last_index = index2;
            break;
        }
    }
    if first_index == 1000000 && last_index == 1000000 {
        return "";
    }

    println!("bytes.len() {}", bytes.len());
    if bytes.len() == 0 {
        println!("IS 0");
        return &s;
    }
    &s[first_index..last_index+1]


}