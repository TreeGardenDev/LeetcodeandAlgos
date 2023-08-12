fn main() {
    let s = 7;
    let t = 11;
    let a = 5;
    let b = 15;
    let apples = [-2, 2, 1];
    let oranges = [5, -6];
    ApplesandOranges(s, t, a, b, &apples, &oranges);
}

fn ApplesandOranges(s:i32, t:i32, a:i32, b:i32, apples:&[i32], oranges:&[i32]){
    let mut count_apple = 0;
    let mut count_orange = 0;
    
    for i in apples.iter(){
       if a+i>=s&&a+i<=t{
           count_apple+=1;
       } 
    }
    for i in oranges.iter(){
        if b+i>=s&&b+i<=t{
            count_orange+=1;
        }
    }

    println!("{}", count_apple);
    println!("{}", count_orange);

}
