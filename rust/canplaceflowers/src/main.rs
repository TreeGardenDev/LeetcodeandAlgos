pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {

    let mut flowerbed=flowerbed;
    let mut count=0;
    let len=flowerbed.len();
    for i in 0..flowerbed.len(){
        if flowerbed[i]==0 && (len ==1 || i==len-1&&flowerbed[i-1]==0 || i==0 && flowerbed[i+1]==0 || i!=0 && i!=len && flowerbed[i-1]==0 &&flowerbed[i+1]==0){
            count+=1;
            flowerbed[i]=1;
        }
    }

    if count as i32 >= n{
        return true;
    }

    return false;
        
}
fn main() {
    let flowerbed = vec![1, 0, 0, 0, 1];
    //let flowerbed=vec![0,0,1,0,1];
   //let flowerbed=vec![0,1,0];
    let result = can_place_flowers(flowerbed, 1);
    println!("result:{}", result);
}
