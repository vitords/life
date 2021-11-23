use std::fmt;
#[derive(Clone)]
struct Life {
    board: [[i32; 3]; 3],
}

impl fmt::Debug for Life {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.board.fmt(f)
    }
}

fn neighbours(life: &mut Life, i: usize, j: usize) -> i32 {
    // i-1,j-1,  i-1,j,  i-1,j+1
    //   i,j-1,            i,j+1
    // i+1,j-1,  i+1,j,  i+1,j+1
    let mut total = 0;

    if i > 0 {
        if j > 0 {
            total += life.board[i-1][j-1];
        } else if j < life.board[i].len() - 1 {
            total += life.board[i-1][j+1];
        }
        if j >= 0 {
            total += life.board[i-1][j];
        }
    }
    if j > 0 {
        total += life.board[i][j-1];
    }
    if j < life.board[i].len() - 1 {
        total += life.board[i][j+1];
    }
    if i < life.board.len() - 1 {
        if j > 0 {
            total += life.board[i+1][j-1];
        }
        total += life.board[i+1][j];
        if j < life.board[i].len() - 1{
            total += life.board[i+1][j+1];
        }
        
    }
    total
}

fn evolve(life: &mut Life) {
    // 1. Any live cell with two or three live neighbours survives.
    // 2. Any dead cell with three live neighbours becomes a live cell.
    // 3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
    let mut new_board = life.board.clone();

    for i in 0..life.board.len() {
        for j in 0..life.board[i].len() {
            let n = neighbours(life, i, j);

            if (n == 2 || n == 3) && life.board[i][j] == 1 {
                new_board[i][j] = 1;
            } else if n == 3 && life.board[i][j] == 0 {
                new_board[i][j] = 1;
            } else {
                new_board[i][j] = 0;
            }
        }
    }
    life.board = new_board;
}

fn main() {
    let mut life = Life{board: [[0; 3]; 3]};
    let generations = 3;

    life.board[1][0] = 1;
    life.board[1][1] = 1;
    life.board[1][2] = 1;
    println!("Seed:\n{:?}", life);

    let mut gen = 0;
    while gen < generations {
        evolve(&mut life);
        println!("Generation {}...", gen);
        println!("{:?}", life);
        gen += 1;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbours() {
        let mut life = Life{board: [[0; 3]; 3]};
        life.board[1][0] = 1;
        life.board[1][1] = 1;
        life.board[1][2] = 1;

        assert_eq!(neighbours(&mut life, 0, 0), 2);
        assert_eq!(neighbours(&mut life, 0, 1), 3);
        assert_eq!(neighbours(&mut life, 1, 1), 2);
        
    }

    #[test]
    fn test_evolve() {
        let mut life = Life{board: [[0; 3]; 3]};
        life.board[1][0] = 1;
        life.board[1][1] = 1;
        life.board[1][2] = 1;

        let mut gen1 = Life{board: [[0; 3]; 3]};
        gen1.board[0][1] = 1;
        gen1.board[1][1] = 1;
        gen1.board[2][1] = 1;

        let gen2 = life.clone();

        evolve(&mut life);
        assert_eq!(life.board, gen1.board);

        evolve(&mut life);
        assert_eq!(life.board, gen2.board)

    }
}
