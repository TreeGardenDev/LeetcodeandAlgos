pub fn reverse_vowels(s: String) -> String {
    let result:String;
    let j = s.len();
    let x = s.chars().collect::<Vec<char>>();
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let mut vowinstring= x.iter().filter(|&c| vowels.contains(c)).collect::<Vec<&char>>();
    let mut stringchar=s.chars().collect::<Vec<char>>();
    for i in 0..j{
        if vowels.contains(&x[i]){
            stringchar[i]=*vowinstring.pop().unwrap();
        }
    }
    result=stringchar.into_iter().collect();

    result
}
fn main() {
    let s = String::from("aA");
    let result = reverse_vowels(s);
    println!("{}", result);
}
