pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut result = 0.00;
    let mut changed = false;

    for i in 0..nums.len() - k as usize + 1 {
        let mut sum = 0.00;

        for j in 0..k as usize {
            if j + i >= nums.len() {
                break;
            }
            sum += nums[j + i] as f64;
        }
        if !changed || sum as f64 > result {
            if !changed {
                changed = true;
            }
            result = sum as f64;
        }
    }
    result / k as f64
}

fn main() {
    //let nums = vec![1, 12, -5, -6, 50, 3];
    //let nums = vec![12, -5, -6, 50];
    let nums = vec![-1];
    let k = 1;
    let result = find_max_average(nums, k);
    println!("result = {}", result);
}
