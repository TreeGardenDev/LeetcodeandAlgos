fn main() {
    let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    let result = trap(height);
    println!("result: {}", result);
}

pub fn trap(height: Vec<i32>) -> i32 {

    let mut result = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    let mut left_max = 0;
    let mut right_max = 0;

    for _ in 0..height.len() {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                result += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                result += right_max - height[right];
            }
            right -= 1;
        }
    }

    result
        
}
