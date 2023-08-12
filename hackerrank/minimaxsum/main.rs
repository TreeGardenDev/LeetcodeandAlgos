fn main (){
    let arr = &[1,2,3,4,5];
    miniMaxSum(arr);

    
}

fn miniMaxSum(arr:&[i32]){
    let mut vec = arr.to_vec().iter().map(|&x| x as i64).collect::<Vec<i64>>();
    vec.sort();
    
    let min=vec.iter().take(4).sum::<i64>();
    let max=vec.iter().skip(1).sum::<i64>();
    
    println!("{} {}",min,max);
    
}
