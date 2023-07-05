pub fn length_of_longest_substring(s: String) -> i32 {
     let mut used=Vec::new();
     let mut max=0;
     let mut current=0;
     let mut last_start=0;
     
     s.chars().enumerate().for_each(|(i,c)| {
         
         if used.contains(&c) {
             if current>max {
                 max=current;
             }

             last_start=used.iter().position(|&x| x==c).unwrap();
             used=used.drain(last_start+1..).collect();
         }
         used.push(c);
         
         current=used.len();
         if max<current {
             max=current;
         }
         
     });
     max as i32
}
fn main() {
    let s = "pwwkew".to_string();
    let result = length_of_longest_substring(s);
    println!("{}", result);

}
