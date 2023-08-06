use std::collections::HashSet;
use std::io;

struct Nqueens {
    n: i32,
}

impl Nqueens {
    fn new(n: i32) -> Nqueens {
        Nqueens { n }
    }

    fn solve_nqueens(&self) -> Vec<Vec<String>> {
        let mut column = vec![];
        let mut row = vec![];
        let mut positive_diagonal = HashSet::new();
        let mut negative_diagonal = HashSet::new();
        let mut res = vec![];

        fn is_valid_placement(r: i32, c: i32, column: &[i32], positive_diagonal: &HashSet<i32>, negative_diagonal: &HashSet<i32>) -> bool {
            !column.contains(&c) && !positive_diagonal.contains(&(r + c)) && !negative_diagonal.contains(&(r - c))
        }

        fn backtrack(r: i32, n: i32, column: &mut Vec<i32>, positive_diagonal: &mut HashSet<i32>, negative_diagonal: &mut HashSet<i32>, row: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if r == n {
                res.push(row.clone());
                return;
            }

            for c in 0..n {
                if is_valid_placement(r, c, &column, &positive_diagonal, &negative_diagonal) {
                    column.push(c);
                    positive_diagonal.insert(r + c);
                    negative_diagonal.insert(r - c);
                    row.push(c);

                    backtrack(r + 1, n, column, positive_diagonal, negative_diagonal, row, res);

                    row.pop();
                    column.pop();
                    positive_diagonal.remove(&(r + c));
                    negative_diagonal.remove(&(r - c));
                }
            }
        }

        backtrack(0, self.n, &mut column, &mut positive_diagonal, &mut negative_diagonal, &mut row, &mut res);
        res.iter().map(|row| {
            row.iter().map(|&c| {
                let mut s = String::new();
                for i in 0..self.n {
                    if i == c as usize as i32 {
                        s.push_str(" Q ");
                    } else {
                        s.push_str(" . ");
                    }
                }
                s
            }).collect()
        }).collect()
    }
}

fn main() {
    println!("Enter the value of n for N-Queens problem:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n: i32 = input.trim().parse().expect("Invalid input");
    let nqueens = Nqueens::new(n);
    let solutions = nqueens.solve_nqueens();

    for solution in solutions {
        for row in solution {
            println!("{}", row);
        }
        println!();
    }
}