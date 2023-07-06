pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        let mut row = vec![0; 9];
        let mut col = vec![0; 9];
        let mut cube = vec![0; 9];
        for j in 0..9 {
            if board[i][j] != '.' {
                let num = board[i][j] as usize - '0' as usize - 1;
                println!("num = {}", num);
                if row[num] == 1 {
                    return false;
                }
                //if col[num] == 1 {
                //    return false;
                //}
                row[num] = 1;
                //col[num] = 1;
            }
            if board[j][i] != '.' {
                let num = board[j][i] as usize - '0' as usize - 1;
                if col[num] == 1 {
                    return false;
                }
                col[num] = 1;
            }
            let row_index = 3 * (i / 3);
            let col_index = 3 * (i % 3);
            if board[row_index + j / 3][col_index + j % 3] != '.' {
                let num = board[row_index + j / 3][col_index + j % 3] as usize - '0' as usize - 1;
                if cube[num] == 1 {
                    return false;
                }
                cube[num] = 1;
            }
        }
    }
    return true;
       
}

fn main() {
    let  board:Vec<Vec<char>>
              =vec![vec!['5','3','.'    ,'.','7','.'    ,'.','.','.'],
                    vec!['6','.','.'    ,'1','9','5'    ,'.','.','.'],
                    vec!['.','9','8'    ,'.','.','.'    ,'.','6','.'],

                    vec!['8','.','.'    ,'.','6','.'    ,'.','.','3'],
                    vec!['4','.','.'    ,'8','.','3'    ,'.','.','1'],
                    vec!['7','.','.'    ,'.','2','.'    ,'.','.','6'],

                    vec!['.','6','.'    ,'.','.','.'    ,'2','8','.'],
                    vec!['.','.','.'    ,'4','1','9'    ,'.','.','5'],
                    vec!['.','.','.'    ,'.','8','.'    ,'.','7','9']];

let result = is_valid_sudoku(board);
println!("result = {}", result);
let board2:Vec<Vec<char>>
               =vec![vec!['8','3','.'   ,'.','7','.'    ,'.','.','.'],
                     vec!['6','.','.'   ,'1','9','5'    ,'.','.','.'],
                     vec!['.','9','8'   ,'.','.','.'    ,'.','6','.'],

                     vec!['8','.','.'   ,'.','6','.'   ,'.','.','3'],
                     vec!['4','.','.'   ,'8','.','3'   ,'.','.','1'],
                     vec!['7','.','.'   ,'.','2','.'   ,'.','.','6'],

                     vec!['.','6','.'   ,'.','.','.'    ,'2','8','.'],
                     vec!['.','.','.'   ,'4','1','9'    ,'.','.','5'],
                     vec!['.','.','.'   ,'.','8','.'    ,'.','7','9']];

    let result2 = is_valid_sudoku(board2);
    println!("result2 = {}", result2);

    let board3:Vec<Vec<char>>
              =vec![vec!['.','.','4'    ,'.','.','.'    ,'6','3','.'],
                    vec!['.','.','.'    ,'.','.','.'    ,'.','.','.'],
                    vec!['5','.','.'    ,'.','.','.'    ,'.','9','.'],

                    vec!['.','.','.'    ,'5','6','.'    ,'.','.','.'],
                    vec!['4','.','3'    ,'.','.','.'    ,'.','.','1'],
                    vec!['.','.','.'    ,'7','.','.'    ,'.','.','.'],

                    vec!['.','.','.'    ,'5','.','.'    ,'.','.','.'],
                    vec!['.','.','.'    ,'.','.','.'    ,'.','.','.'],
                    vec!['.','.','.'    ,'.','.','.'    ,'.','.','.']];
    let result3 = is_valid_sudoku(board3);
    println!("result3 = {}", result3);
}
