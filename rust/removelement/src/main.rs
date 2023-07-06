pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    for i in (0..nums.len()).rev() {
        if nums[i] == val {
            nums.remove(i);
        }

        
    }
    nums.len() as i32
}

fn main() {
    let mut nums = vec![3,2,2,3];
    let val = 3;
    let result = remove_element(&mut nums, val);
    println!("result: {:?}", result);
}
