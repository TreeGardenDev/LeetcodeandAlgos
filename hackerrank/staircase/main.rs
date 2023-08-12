fn main(){
    let count=6;
    staircase(count);


}

fn staircase(count:i32){
    for i in 1..count+1{
        for _ in 0..count-i{
            print!(" ");
        }
        for _ in 0..i{
            print!("#");
        }
        println!();
    }    
}
