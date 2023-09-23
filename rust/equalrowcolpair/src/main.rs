pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut count = 0;

    for i in 0..len {
        let mut continuous = 0;

        for j in 0..len {
            if grid[i][j] == grid[j][i] {
                println!("j: {:?}", grid[i][j]);
                continuous += 1;
            } else {
                break;
            }
            if continuous == len {
                println!("Got one");
                count += 1;
            }
        }

        //let mut continuous = 0;
        //for j in 0..len {
        //    if grid[i][j] == grid[j][i] {
        //        println!("{} {}", grid[i][j], grid[j][i]);

        //        continuous += 1;
        //    } else {
        //        continuous = 0;
        //        continue;
        //    }

        //    if continuous >= len {
        //        count += 1;
        //        println!("Got one");

        //        break;
        //    }
        //}
    }

    return count;
}

fn main() {
    //let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
    let grid = vec![
        vec![3, 1, 2, 2],
        vec![1, 4, 4, 5],
        vec![2, 4, 2, 2],
        vec![2, 4, 2, 2],
    ];
    let result = equal_pairs(grid);
    println!("{}", result);
}
