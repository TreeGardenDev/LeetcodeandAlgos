fn main() {
    let digits = "23".to_string();
    let result = letter_combinations(digits);
    println!("{:?}", result);
}

pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut result = Vec::new();

    let mut map = std::collections::HashMap::new();
    map.insert('2', vec!['a', 'b', 'c']);
    map.insert('3', vec!['d', 'e', 'f']);
    map.insert('4', vec!['g', 'h', 'i']);
    map.insert('5', vec!['j', 'k', 'l']);
    map.insert('6', vec!['m', 'n', 'o']);
    map.insert('7', vec!['p', 'q', 'r', 's']);
    map.insert('8', vec!['t', 'u', 'v']);
    map.insert('9', vec!['w', 'x', 'y', 'z']);
    map.insert('0', vec![' ']);

    for i in 0..digits.len() {
        let digit = digits.chars().nth(i).unwrap();
        let chars = map.get(&digit).unwrap();
        if result.len() == 0 {
            for c in chars {
                result.push(c.to_string());
            }
        } else {
            let mut temp = Vec::new();
            for s in result {
                for c in chars {
                    let mut s = s.clone();
                    s.push(*c);
                    temp.push(s);
                }
            }
            result = temp;
        }
    }
    result
        
    }
