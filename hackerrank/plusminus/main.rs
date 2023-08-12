fn plusminus(arr: &[i32]){
    let len=arr.len() as f32;
    let mut positive=0 as f32;
    let mut negative=0 as f32;
    let mut zero=0 as f32;

    for &i in arr{
        if i>0{
            positive+=1.0;
        }else if i<0{
            negative+=1.0;
        }else{
            zero+=1.0;
        }
    }

    println!("{}", positive/len); 
    println!("{}", negative/len); 
    println!("{}",zero/len); 
}


fn main(){
    let arr=&[1,-1,-1,-1,0,1];

    plusminus(arr);
    
}
