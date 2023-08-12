use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let mut sum1 = 0;
    let mut sum2 = 0;
    let square_size = arr.len();


    for i in 0..square_size {
        sum1 += arr[i][i];
        sum2 += arr[i][square_size - i - 1];
    }
    (sum1 - sum2).abs()
    
}

fn main() {
    //let stdin = io::stdin();
    //let mut stdin_iterator = stdin.lock().lines();

    //let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    //let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    //let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    //for i in 0..n as usize {
    //    arr.push(Vec::with_capacity(n as usize));

    //    arr[i] = stdin_iterator.next().unwrap().unwrap()
    //        .trim_end()
    //        .split(' ')
    //        .map(|s| s.to_string().parse::<i32>().unwrap())
    //        .collect();
    //}

    let mut arr = vec![vec![6,6,7,-10,9,-3,8,9,-1],
                       vec![9,7,-10,6,4,1,6,1,1],
                       vec![-1,-2,4,-6,1,-4,-6,3,9],
                       vec![-8,7,6,-1,-6,-6,6,-7,2],
                       vec![-10,-4,9,1,-7,8,-5,3,-5],
                       vec![-8,-3,-4,2,-3,7,-5,1,-5],
                       vec![-2,-7,-4,8,3,-1,8,2,3],
                       vec![-3,4,6,-7,-7,-8,-3,9,-6],
                       vec![-2,0,5,4,4,4,-3,3,0]];
    
    

    let result = diagonalDifference(&arr);
    println!("{}", result);

}

