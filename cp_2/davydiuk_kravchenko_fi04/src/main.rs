use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::io::Read;

const DIR: &str = "D:\\kpi\\symcryptology\\lab\\cp_2\\Potuzhnyi_Svirsh_fi03_cp_2\\src\\main\\resources\\";
const CHARSET: &'static str = "абвгдежзийклмнопрстуфхцчшщъыьэюя";
static mut freq: HashMap<char, f64> = HashMap::new();
struct CypherText ;

impl CypherText{
    fn letter_probability(file_name: &str) -> HashMap<char, f64> {
        let mut file = File::open(file_name).unwrap();
        let mut file_text = String::new();
        file.read_to_string(&mut file_text).unwrap();
        let file_text = file_text.to_lowercase().chars().filter(|c| Self::CHARSET.contains(*c)).collect::<String>();
        let mut letter_count = HashMap::new();

        for c in Self::CHARSET.chars() {
            letter_count.insert(c, file_text.chars().filter(|&x| x == c).count());
        }

        let total_letters = file_text.len() as f64;
        letter_count.into_iter().map(|(c, count)| (c, count as f64 / total_letters)).collect()
    }

    unsafe fn init() {
        freq = letter_probability(&(DIR.to_owned() + "big"));
    }
}

struct VigenereCipher;

impl VigenereCipher {
    fn encode(text: &str, key: &str, charset: &str) -> String {
        let upper_key = key.to_lowercase();
        let upper_text = text.to_lowercase();
        upper_text.chars().enumerate().map(|(index, c)| {
            let key_index = charset.find(upper_key.chars().nth(index % upper_key.len()).unwrap()).unwrap();
            let text_index = charset.find(c).unwrap();
            charset.chars().nth((text_index + key_index) % charset.len()).unwrap()
        }).collect()
    }

    pub fn decode(text: &str, key: &str, charset: &str) -> String {
        let upper_key = key.to_lowercase();
        let upper_text = text.to_lowercase();
        upper_text.chars().enumerate().map(|(index, c)| {
            let key_index = charset.find(upper_key.chars().nth(index % upper_key.len()).unwrap()).unwrap();
            let text_index = charset.find(c).unwrap();
            charset.chars().nth((text_index - key_index + charset.len()) % charset.len()).unwrap()
        }).collect()
    }
}

fn n_t(text: &str, c: char) -> usize {
    text.chars().filter(|&x| x == c).count()
}

fn m_i(block: &str, g: usize, charset: &str) -> f64 {
    CypherText.freq.iter().map(|(char, freq)| {
        freq * n_t(block, charset.chars().nth((charset.find(*char).unwrap() + g) % charset.len()).unwrap()) as f64
    }).sum()
}

fn i(text: &str, charset: &str) -> f64 {
    let mut sum = 0.0;
    for c in charset.chars() {
        let count = n_t(text, c);
        sum += count as f64 * (count - 1) as f64;
    }
    sum / (text.len() as f64 * (text.len() - 1) as f64)
}

fn get_r(text: &str, start: usize, end: usize, charset: &str) -> HashMap<usize, f64> {
    let mut result = HashMap::new();
    for r in start..=end {
        let mut blocks = vec![String::new(); r];
        for (i, c) in text.chars().enumerate() {
            blocks[i % r].push(c);
        }
        let avg_i = blocks.iter().map(|block| i(block, charset)).sum::<f64>() / r as f64;
        result.insert(r, avg_i);
    }
    result
}


fn main() {
    let open_text = {
        let mut file = File::open(&(DIR.to_owned() + "fileToEncrypt")).unwrap();
        let mut file_text = String::new();
        file.read_to_string(&mut file_text).unwrap();
        file_text = file_text.to_lowercase().chars().filter(|c| CypherText::CHARSET.contains(*c)).collect();
        file_text
    };

    let keys = vec!["ХЛ", "ЗЛО", "ХЛПК", "ПТХХЛ", "ЙОБАНАРУСНЯ".to_lowercase()];
    let vigenere = VigenereCipher;

    let encodings = keys.iter().map(|key| vigenere.encode(&open_text, key)).collect::<Vec<String>>();

    let output_file_path = DIR.to_owned() + "output";

    let mut output_file = File::create(&output_file_path).unwrap();
    writeln!(output_file, "Encoded and Decoded texts:").unwrap();
    for (index, encoding) in encodings.iter().enumerate() {
        let decoded = vigenere.decode(encoding, &keys[index]);
        writeln!(output_file, "Text {}:", index + 1).unwrap();
        writeln!(output_file, "Encoded: {}", encoding).unwrap();
        writeln!(output_file, "Decoded: {}", decoded).unwrap();
    }

    let open_r = i(&open_text, &CypherText::CHARSET);
    let r_values = encodings.iter().map(|encoding| i(encoding, &CypherText::CHARSET)).collect::<Vec<f64>>();

    writeln!(output_file, "Ir open:").unwrap();
    writeln!(output_file, "{}", open_r).unwrap();
    for (index, r) in r_values.iter().enumerate() {
        writeln!(output_file, "Ir {}: ", keys[index].len()).unwrap();
        writeln!(output_file, "{}", r).unwrap();
    }

    let cypher_file_name = DIR.to_owned() + "cipher_var15";
    let cypher = {
        let mut file = File::open(&cypher_file_name).unwrap();
        let mut file_text = String::new();
        file.read_to_string(&mut file_text).unwrap();
        file_text = file_text.to_lowercase().chars().filter(|c| CypherText::CHARSET.contains(*c)).collect();
        file_text
    };

    let match_dict = get_r(&cypher, 1, 30, &CypherText::CHARSET);
    let mut match_list: Vec<_> = match_dict.iter().collect();
    match_list.sort_by(|(_, val1), (_, val2)| (val1 - open_r).abs().partial_cmp(&(val2 - open_r).abs()).unwrap_or(Ordering::Equal));

    println!("for encoded text");
    println!("{:?}", match_dict);
    println!("{:?}", match_list);
    let best_fit = match_list[0].0;

    let mut blocks = vec!["".to_string(); *best_fit];
    for (i, c) in cypher.chars().enumerate() {
        blocks[i % best_fit].push(c);
    }

    let sorted_probabilities = {
        let mut freq_vec = CypherText::freq.iter().collect::<Vec<_>>();
        freq_vec.sort_by(|(_, val1), (_, val2)| val2.partial_cmp(val1).unwrap_or(Ordering::Equal));
        freq_vec
    };

    let top_10_chars: Vec<char> = sorted_probabilities.iter().take(10).map(|(char, _)| *char).collect();

    let mut keys_result = Vec::new();
    for most_possible in top_10_chars {
        let mut key = String::new();
        for i in 0..best_fit {
            let val_map = CypherText::CHARSET.chars().map(|char| (char, n_t(&blocks[i], char))).collect::<HashMap<_, _>>();
            let max_key = val_map.iter().max_by_key(|(_, &count)| count).map(|(&char, _)| char).unwrap();
            let k = (CypherText::CHARSET.find(max_key).unwrap() - CypherText::CHARSET.find(most_possible).unwrap() + CypherText::CHARSET.len()) % 32;
            key.push(CypherText::CHARSET.chars().nth(k).unwrap());
        }
        keys_result.push(key);
    }
    println!("{:?}", keys_result);

    let key_m = blocks.iter().map(|block| {
        let val_map = CypherText::CHARSET.chars().map(|char| (char, m_i(block, CypherText::CHARSET.find(char).unwrap(), &CypherText::CHARSET))).collect::<HashMap<_, _>>();
        *val_map.iter().max_by_key(|(_, &val)| val).map(|(&char, _)| char).unwrap()
    }).collect::<String>();

    println!("{}", key_m);
    println!("{}", vigenere.decode(&cypher, &key_m));
}