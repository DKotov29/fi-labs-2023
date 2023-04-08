use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;

fn main() {
    let file_content = fs::read_to_string("TEXT").expect("problem with reading of file");

    let filtered = file_content
        .chars()
        .filter(|char| char.is_alphabetic())
        .collect::<String>();
    let mut set_chars = HashSet::new();
    let mut char_amount_map = HashMap::new();
    let mut amount = 0usize;
    for char in filtered.chars() {
        let char = char.to_lowercase().next().unwrap();
        set_chars.insert(char);
        char_amount_map
            .entry(char)
            .and_modify(|counter| *counter += 1)
            .or_insert(1usize);
        amount += 1;
    }
    println!("{set_chars:?}\n{amount}");

    println!("{char_amount_map:?}");
    let mut char_percent_map = char_amount_map
        .iter()
        .map(|(char, counter)| (*char, *counter as f64 / amount as f64))
        .collect::<HashMap<char, f64>>();
    println!("{char_percent_map:?}");
    let one: f64 = char_percent_map.iter().map(|(f1, f2)| *f2).collect::<Vec<f64>>().iter().sum();
    println!("{}", one);

    let mut char_per_sorted: Vec<(char, f64)> = char_percent_map.clone().into_iter().collect();
    char_per_sorted.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    println!("{char_per_sorted:?}");

}
