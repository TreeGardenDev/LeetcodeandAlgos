pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let totalproduct=nums.iter().fold(1,|acc,x| acc*x);
    let len=nums.len();
    let mut result:Vec<i32>=Vec::new();
    for i in 0..len{
        if nums[i]!=0{

            result.push(totalproduct/nums[i]);
        }else{
            //enumerate
            result.push(nums.iter().enumerate().filter(|x| x.0!=i).fold(1, |acc,x|acc*x.1) as i32);
        }
    }

    result
        
}
fn main() {
    let nums = vec![1, 2, 3, 4];
    let nums=vec![-1,1,0,-3,3];
    let nums=vec![0,0];
    //let nums=vec![0,4,0];
    let result = product_except_self(nums);
    println!("{:?}", result);
}
