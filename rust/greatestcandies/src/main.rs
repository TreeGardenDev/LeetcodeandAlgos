pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let result= candies.iter().map(|x| x+extra_candies >= *candies.iter().max().unwrap()).collect();
    result
}
fn main() {
    let candies = vec![2,3,5,1,3];
    let extra_candies = 3;
    let result = kids_with_candies(candies, extra_candies);
    println!("{:?}", result);
}

