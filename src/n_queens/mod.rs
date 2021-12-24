#![allow(unused)]
struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        // 初始化空棋盘，“.”表示空，“Q”表示皇后
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut solution = vec![];
        backtrack(&mut board, &mut solution, n, 0);
        solution
    }
}

pub fn backtrack(board: &mut Vec<Vec<char>>, solution: &mut Vec<Vec<String>>, n: i32, row: i32) {
    // 循环所有列查找棋子放置方式
    for column in 0..n {
        // 判断row行column列放置棋子是否合适
        if !collision(&board, n, row, column) {
            // 做选择
            board[row as usize][column as usize] = 'Q';
            // 递归终止条件
            if row == n - 1 {
                // 记录结果
                solution.push(board.iter().map(|vec| vec.iter().collect()).collect());
            } else {
                // 递归调用
                backtrack(board, solution, n, row + 1);
            }
            // 撤销选择
            board[row as usize][column as usize] = '.';
        }
    }
}

// 判断row行column列放置棋子是否合适
fn collision(board: &Vec<Vec<char>>, n: i32, row: i32, column: i32) -> bool {
    let mut up_row = row - 1; // 往上一行
    let mut left_column = column - 1; // 往左一列
    let mut right_column = column + 1; // 往右一列

    // 逐行往上考察每一行
    while up_row >= 0 {
        // 考察column列是否已存在Q，若存在则冲突
        if board[up_row as usize][column as usize] == 'Q' {
            return true;
        }
        // 考察左上对角线是否已存在Q，若存在则冲突
        if left_column >= 0 && board[up_row as usize][left_column as usize] == 'Q' {
            return true;
        }
        // 考察右上对角线是否已存在Q，若存在则冲突
        if right_column < n && board[up_row as usize][right_column as usize] == 'Q' {
            return true;
        }

        up_row -= 1; // 继续往上一行
        left_column -= 1; // 继续往左一列
        right_column += 1; // 继续往右一列
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_combinations() {
        let n = 4;
        let result = vec![
            vec![".Q..", "...Q", "Q...", "..Q."],
            vec!["..Q.", "Q...", "...Q", ".Q.."],
        ];
        assert_eq!(Solution::solve_n_queens(n), result);
    }
}
