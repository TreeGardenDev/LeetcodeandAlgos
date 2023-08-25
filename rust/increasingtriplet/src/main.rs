pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let len = nums.len();

    let mut result = false;
    let mut first= i32::MAX;
    let mut second= i32::MAX;

    for i in 0..len{
        if nums[i]<=first{
            first=nums[i];
        }else if nums[i]<=second{
            second=nums[i];
        }
        else{
            return true
        }
        
    }
    result
}
//had to look this up question very unclear on what the parameters are
 pub fn increasing_triplet2(nums: Vec<i32>) -> bool 
    {
        if nums.len() < 3 { return false; }
        
        // [1] there exist a triple of increasing numbers if some
        //     number 'n' is greater than another number 'second'
        //     that itself is greater than yet another number 'first'
        
        let mut first  = i32::MAX;
        let mut second = i32::MAX;
        
        // [2] in the cycle, we iteratively update 'first' and 'second'
        //     numbers while waiting for the third one to appear
        for n in nums
        {
            if      n <= first  { first  = n; }
            else if n <= second { second = n; }
            else                { return true; }
        }
        
        return false;
    }

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let nums = vec![5, 4, 3, 2, 1];
    let nums=vec![2,1,5,0,4,6];
    let nums=vec![1,1,-2,6];
    let nums=vec![20,100,10,12,5,13];
    let nums=vec![2,4,-2,-3];
    //let nums=vec![1,2,1,3];
    //let nums=vec![1,2];
    let result = increasing_triplet(nums);
    println!("result: {}", result);
}
        //if i==len-1{
        //    if nums[i]<nums[0]&&nums[0]<nums[1]{
        //        result=true;
        //    }
        //}
