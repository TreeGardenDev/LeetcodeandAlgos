pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut result = 0;

    let mut current = 0;
    for i in gain {
        println!("new delta: {}", i);

        current = current + i;
        println!("new current: {}", current);

        if current > result {
            result = current;
        }
    }

    result
}
fn main() {
    let gain = vec![1, 2, 3];
    let result = largest_altitude(gain);
    println!("{}", result);
}
