use std::collections::HashMap;

fn main() {
    word_counter("apple apple string pear pear cat dog");
}

fn word_counter(list: &str) -> HashMap {
    let mut word_count = HashMap::new();

    for words in list.split(" ") {
        word_count
            .entry(words)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    println!(word_count);
}
