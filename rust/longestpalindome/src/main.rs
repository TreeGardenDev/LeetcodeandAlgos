 pub fn longest_palindrome(s: String) -> String {
     let mut s1=s.clone();
     let s2=s1.chars().rev().collect::<String>();
     let mut last_start=0;
     let mut counter=0;
     let mut palindromes=Vec::new();

     if s1==s2{
         return s1;
     }


     //s1.chars().rev().collect::<String>()
     s1.chars().enumerate().for_each(|(i,c)|{
         counter+=1;

         println!("{} {}",c,palindromes.len());
         println!("{}",counter);
            if counter%2==0 && c != palindromes[palindromes.len()-(counter-1)]{
                palindromes.push(c);
                last_start=palindromes.iter().position(|&x| x==c).unwrap();

                palindromes=palindromes.drain(last_start+1..).collect();
                
            }
            else{
                palindromes.push(c);
            }

     });
     let s3=palindromes.iter().collect::<String>();
     s3
        
    }
fn main() {
    let s=String::from("babad");
    let s1=longest_palindrome(s);
    println!("{}",s1);
}
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
