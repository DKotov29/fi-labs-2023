extern crate core;

use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let mut file_content = fs::read_to_string("TEXT").expect("problem with reading of file");
    let filtered = file_content
        .chars()
        .filter(|char| "абвгдеёжзийклмнопрстуфхцчшщъыьэюяАБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ".contains(|e| e == *char) //|| char == &' '
        )
        .map(|char| {
            match char {
                'ё' => 'е',
                'ъ' => 'ь',
                _ => char,
            }
        })
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
    let one: f64 = char_percent_map
        .iter()
        .map(|(f1, f2)| *f2)
        .collect::<Vec<f64>>()
        .iter()
        .sum();
    println!("{}", one);

    let mut char_per_sorted: Vec<(char, f64)> = char_percent_map.clone().into_iter().collect();
    char_per_sorted.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    println!("{char_per_sorted:?}");
    let h1 = -char_per_sorted.iter().map(|(a,b )| *b * b.log2()).sum::<f64>();
    println!("H1 chars =  {}", h1);
    let h2 = h1 * 0.5;
    println!("H2 chars =  {}", h2);

    let chars: Vec<char> = filtered.chars().collect();

    let mut bigram1 = chars
        .chunks(2)
        .map(|f| String::from_iter(f).to_lowercase())
        .collect::<Vec<String>>();
    if bigram1.iter().last().unwrap().len() == 1 {
        bigram1.remove(bigram1.len());
    }
    let mut bigram_map1 = HashMap::new();
    bigram1.iter().for_each(|bigram| {
        bigram_map1
            .entry(bigram.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(1usize);
    });
    println!("amount: {}, {:?}",bigram_map1.len(),  bigram_map1);

    let mut bigram_chastota_map1 = bigram_map1.iter()
        .map(|(s, amount)| (s.clone(), *amount as f64 / bigram1.len() as f64)).collect::<Vec<(String,f64)>>();
    println!("{:?}", bigram_chastota_map1);

    let mut bigram1mapsorted: Vec<(String, f64)> = bigram_chastota_map1.clone();
    bigram1mapsorted.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    println!("{bigram1mapsorted:?}");

    println!("{}", bigram1mapsorted.iter().map(|(a,b)| *b).sum::<f64>());

    let h1 = -bigram1mapsorted.iter().map(|(a,b )| *b * b.log2()).sum::<f64>();
    println!("H1 bigram1 =  {}", h1);
    let h2 = h1 * 0.5;
    println!("H2 bigram1 =  {}", h2);


    let mut bigram2 = Vec::new();
    for i in 0..chars.len()-1 {
        let mut bigram = String::new();
        bigram.push(chars[i]);
        bigram.push(chars[i+1]);
        bigram2.push(bigram.to_lowercase());
    }

    let mut bigram_map2 = HashMap::new();
    bigram2.iter().for_each(|bigram| {
        bigram_map2
            .entry(bigram.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(1usize);
    });
    println!("amount: {}, {:?}",bigram_map2.len(),  bigram_map2);

    let mut bigram_chastota_map2 = bigram_map2.iter()
        .map(|(s, amount)| (s.clone(), *amount as f64 / bigram1.len() as f64)).collect::<Vec<(String,f64)>>();
    println!("{:?}", bigram_chastota_map2);

    let mut bigram2mapsorted: Vec<(String, f64)> = bigram_chastota_map2.clone();
    bigram2mapsorted.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    println!("{bigram1mapsorted:?}");
    let h1 = -bigram2mapsorted.iter().map(|(a,b )| *b * b.log2()).sum::<f64>();
    println!("H1 bigram2 =  {}", h1);
    let h2 = h1 * 0.5;
    println!("H2 bigram2 =  {}", h2);

}
