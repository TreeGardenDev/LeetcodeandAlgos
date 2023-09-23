fn main() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;
    let result = three_sum_closest(nums, target);

    println!("result: {}", result);
}

pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut closest=i32::MIN;

    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            for k in j+1..nums.len() {
                let sum=nums[i]+nums[j]+nums[k];
                if closest==i32::MIN || (sum-target).abs()<(closest-target).abs() {
                    closest=sum;
                }
            
            }
        }
        
    }
    closest
    
}
