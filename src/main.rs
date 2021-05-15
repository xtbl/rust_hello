use std::env;
use std::fs;

fn main() {

    // [x] create list of names
    // [x] read params (path_file, search_name)
    // [x] read list
    // [x] check if search_name was found
    // [x] print message if name is found

    let path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();
    println!("arg1 {}", path);
    println!("arg1 {}", search_name);
    // has both params
    assert_eq!(path, "moonwalkers.txt");
    assert_eq!(search_name, "Scott");

    println!("{:?}", std::env::current_exe());
    let file_content = fs::read_to_string("moonwalkers.txt").unwrap();
    assert_eq!(is_name_in_list(file_content, String::from("Armstrong")), true);

    let file_content = fs::read_to_string("moonwalkers.txt").unwrap();
    if is_name_in_list(file_content, search_name) {
      println!("Name was found in list.");
    }

    println!("Tests passed!");
}

fn is_name_in_list(list: String, name: String) -> bool {
  let mut is_in_list: bool = false;
  for name_line in list.lines() {
    if name_line == name {
      is_in_list = true;
    }
  }
  is_in_list
}