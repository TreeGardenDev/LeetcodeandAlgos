pub fn length_of_last_word(s: String) -> i32 {
        
    let vecwordsnospace: Vec<&str> = s.split_whitespace().collect();

    println!("{:?}", vecwordsnospace);
    let lastword = vecwordsnospace.last();
    let lastlength = lastword.unwrap().len() as i32;
    lastlength

    }
fn main() {
    let s = "Hello World".to_string();
    let result = length_of_last_word(s);
    println!("{}", result);
}
