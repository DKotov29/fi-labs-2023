use std::collections::HashSet;
// fn main() {
//     let russian_char = 'б'; // The starting Russian character
//
//     let incremented_char = char::from_u32(((russian_char as u32) + 2)).unwrap(); // Increment the Unicode scalar value by 2
//
//     println!("Original: {}", russian_char);
//     println!("Incremented: {}", incremented_char);
// }
use std::fs;


fn encrypt(key: &str) -> String {
    let mut buffff = String::new();
    let mut file_content = fs::read_to_string("TEXT").expect("problem with reading of file");
    let mut result = String::new();
    let mut key = key.to_string();

    let key_len = key.chars().count();
    let mut alph = "абвгдежзийклмнопрстуфхцчшщъыьэюя";
    key = key.to_lowercase();
    file_content = file_content.to_lowercase();
    // println!("file: \"{file_content}\"");
    {
        let mut re = String::new();
        for k in key.chars() {
            if k == 'ё' {
                re.push('е');
            } else {
                if alph.contains(|p| p == k) {
                    re.push(k);
                }
            }
        }
        key = re;
        re = String::new();
        for k in file_content.chars() {
            if k == 'ё' {
                re.push('е');
            } else {
                if alph.contains(|p| p == k) {
                    re.push(k);
                }
            }
        }
        file_content = re;
    }
    println!("index: {}", index(file_content.as_str()));
    println!("text: {file_content}\nkey: {key}");
    // println!("char + key_char = result");
    let alph_len = alph.chars().count();

    for (i, char) in file_content.chars().enumerate() {
        if alph.contains(|p| p == char) {
            let mut char = char;


            let key_char = match key.chars().nth(i % key_len) {
                None => {
                    println!("{key}");
                    println!("{i} {key_len} {}", i % key_len);
                    panic!();
                }
                Some(c) => { c }
            };
            let char_pos = alph.chars().position(|p| p == char).unwrap();
            //println!("{char_pos} {key_char}");
            let key_char_pos = alph.chars().position(|p| p == key_char).unwrap();
            //println!("char_pos: {char_pos} + key_char_pos {key_char_pos}) = alph_len {alph_len}");
            let res_pos = (char_pos + key_char_pos + 1) % alph_len;
            let res = alph.chars().nth(res_pos).unwrap();
            result.push(res);
        }
    }

    println!("result: {result}");
    result
}

fn index(text: &str) -> f64 {
    let mut set = HashSet::new();
    for i in text.chars() {
        set.insert(i);
    }
    let alph_symb_in_text = set.len();
    let text_symb_count = text.chars().count();
    dbg!(text_symb_count);
    dbg!(alph_symb_in_text);
    let mut v = Vec::new();
    for char in set {
        let mut c = 0usize;
        for i in text.chars() {
            if i == char {
                c += 1;
            }
        }
        v.push((char, c));
    }
    dbg!(&v);
    let res = (1f64 / (text_symb_count as f64 * (text_symb_count as f64 - 1f64))) * ((v.iter().map(|(_, count)| count * (count - 1)).sum::<usize>()) as f64);
    res
}

#[cfg(test)]
mod tests {
    use crate::encrypt;

    #[test]
    fn check_key1() {
        let key = "не";
        encrypt(key);
        assert!(true);
    }

    #[test]
    fn check_key2() {
        let key = "так";
        encrypt(key);
        assert!(true);
    }

    #[test]
    fn check_key3() {
        let key = "може";
        encrypt(key);
        assert!(true);
    }

    #[test]
    fn check_key4() {
        let key = "навря";
        encrypt(key);
        assert!(true);
    }

    #[test]
    fn check_key5() {
        let key = "можливиййа";
        encrypt(key);
        assert!(true);
    }

    #[test]
    fn check_key6() {
        let key = "сьогоднируснягоритьа";
        encrypt(key);
        assert!(true);
    }

    #[test]
    fn check_key7() {
        let key = "СЬОГОДНИРУСНЯГОРИТЬА";
        encrypt(key);
        assert!(true);
    }
}

fn main() {}