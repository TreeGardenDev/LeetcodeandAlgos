pub fn reverse_words(s: String) -> String {
    let mut result=String::new();
    let mut words:Vec<&str>=s.split(" ").filter(|&x| x!="").collect();
    let _=words.reverse();
    let len=words.len();
    for i in 0..len{
       result.push_str(words[i]);
       if i !=len-1{
              result.push_str(" ");
       }
    }

    result
}
fn main() {
    let s=String::from("  The sky is  blue  ");
    println!("{}", s);
    let result=reverse_words(s);
    println!("{}",result);
}
