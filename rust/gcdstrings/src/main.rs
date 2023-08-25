pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let result=String::new();
    //let larger_str = if str1.len() > str2.len() { str1 } else { str2 };
    if str1.len()%2!=0 && str2.len()%2 != 0{
        return result
    }
    let smaller_str = if str1.len() > str2.len() { str2.clone() } else { str1.clone() };
    let factor=str1.len() as i32/str2.len() as i32;
    println!("factor is {}",factor);
    let initial=1;

    
        
    result
}
fn main() {
    let str1 = "ABCABC".to_string();
    let str2 = "ABC".to_string();
    let result = gcd_of_strings(str1, str2);
}
