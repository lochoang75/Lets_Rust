use std::collections::HashMap;

fn main() {
    let text = "Rust is fast, reliable, and memory-efficient. Rust helps developers build software that runs blazingly fast without bugs.";

    // Call your word frequency function here
    let frequencies = word_frequencies(text);

    // Print results
    for (word, count) in frequencies {
        println!("{}: {}", word, count);
    }
}

fn word_frequencies(text: &str) -> Vec<(String, usize)> {
    let mut tmp_map = HashMap::new();

    // TODO: Normalize case, remove punctuation, and split into wordsjj
    let lower_case =  text.to_lowercase();
    for sub_str in lower_case.split_ascii_whitespace()
    {
        tmp_map.entry(sub_str.replace(",", "").replace(".", "")).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut vec: Vec<(String, usize)> = tmp_map.into_iter().collect();
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    vec
}