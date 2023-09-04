pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    let mut pairs: Vec<(usize)> = Vec::new();
    for i in 0..nums.len() {
        for j in 1..nums.len() {
            if i == j {
                continue;
            }
            if nums[i] + nums[j] == k {
                if pairs.contains(&i) || pairs.contains(&j) {
                    continue;
                }
                pairs.push(i);
                pairs.push(j);
                count += 1;
            }
        }
    }

    count
}
fn main() {
    //let nums = vec![1, 2, 3, 4];
    let nums = vec![3, 5, 1, 5];
    let nums = vec![4, 4, 1, 3, 1, 3, 2, 2, 5, 5, 1, 5, 2, 1, 2, 3, 5, 4];

    let result = max_operations(nums, 2);
    println!("result: {}", result);
}
