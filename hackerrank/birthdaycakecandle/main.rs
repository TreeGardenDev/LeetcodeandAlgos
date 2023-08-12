fn main() {
    let candles=&[3,2,1,3];

    let result=birthdaycakeCandles(candles);
    println!("Result: {}",result);


}

fn birthdaycakeCandles(arr:&[i32])->i32{
    let max=arr.iter().max().unwrap();
    let count=arr.iter().filter(|&x| x==max).count();
    count as i32
    
}
