pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut count=0;
    let mut i=0;
    while i < nums.len(){
        if nums[i]==0{
            nums.remove(i);
            count+=1;
        }
        else{
            i+=1;
        }
    }
    //println!("{:?}",nums);
    //println!("{}",count);
    nums.append(&mut vec![0;count]);
}
fn main() {
    //let mut nums = vec![0,1,0,3,12];
    let mut nums=vec![0,0,1];
    //println!("{:?}",nums);
    move_zeroes(&mut nums);
    //println!("{:?}", nums);
}
