pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut vec1not2 = Vec::new();
    let mut vec2not1 = Vec::new();
    for i in nums1.iter() {
        if !nums2.contains(i) && !vec1not2.contains(i) {
            vec1not2.push(*i);
        }
    }
    for i in nums2.iter() {
        if !nums1.contains(i) && !vec2not1.contains(i) {
            vec2not1.push(*i);
        }
    }
    result.push(vec1not2);
    result.push(vec2not1);

    result
}

fn main() {
    let nums1 = vec![1, 2, 3];

    let nums2 = vec![1, 2, 3, 4];
    let result = find_difference(nums1, nums2);
    println!("{:?}", result);
}
