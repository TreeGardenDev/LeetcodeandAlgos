fn main() {
    let nums = vec![1, 2, 3];
    let result = permute(nums);
    println!("{:?}", result);
}

 pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    let mut path = Vec::new();
    let mut used = vec![false; nums.len()];

    dfs(&nums, &mut path, &mut used, &mut result);
    result

    }
fn dfs(nums: &Vec<i32>, path: &mut Vec<i32>, used: &mut Vec<bool>, result: &mut Vec<Vec<i32>>) {
    if path.len() == nums.len() {
        result.push(path.clone());
        return;
    }
    for i in 0..nums.len() {
        if !used[i] {
            path.push(nums[i]);
            used[i] = true;
            dfs(nums, path, used, result);
            used[i] = false;
            path.pop();
        }
    }
}
