pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area:i32=0;
    let mut left=0;
    let mut right=height.len()-1;
    while left<right{
        let area=(right-left) as i32 * std::cmp::min(height[left],height[right]);
        if area>max_area{
            max_area=area;
        }
        if height[left]<height[right]{
            left+=1;
        }else{
            right-=1;
        }
    }

    max_area
}
fn main() {
    let watervec=vec![1,8,6,2,5,4,8,3,7];
    let maxarea=max_area(watervec);
    println!("maxArea:{}",maxarea);
}
