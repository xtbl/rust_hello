use std::io;
use std::env;
use std::fs;
use std::collections::HashMap;

// read file
    // receive path as command arg
// count number of times each word occurs
    // parse text into words split_whitespace()
    // ignore capitalization
    // keep track of how many times each unique word occurs
    // there might be multiple most common words
// print message with most common words and how many times they appeared

fn main() {

    let path = env::args().nth(1).unwrap();
    println!("arg1 {}", path);
    assert_eq!(path, "word_list.txt");

    let file_content = fs::read_to_string(path).unwrap();
    println!("file content {}", file_content);

    let mut splitted = file_content.split_whitespace();
    assert_eq!(Some("Gossip"), splitted.next());
    assert_eq!(Some("was"), splitted.next());

    // println!("insert_words: {:?}", insert_words(splitted));
    let mut word_freqs = insert_words(splitted);
    // sort hashmap
    let mut map_to_vec: Vec<(&String, &u32)> = word_freqs.iter().collect();
    map_to_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("map_to_vec sorted: {:?}", map_to_vec);

    println!("Tests passed!");
}

fn insert_words(words: std::str::SplitWhitespace<'_>) -> HashMap<String, u32> {
    let mut word_freq = HashMap::new();
    for word in words {
        println!("word: {}", word);
        let word_freq_ref = word_freq.entry(String::from(word)).or_insert(0);
        *word_freq_ref += 1;
    }
    word_freq
}