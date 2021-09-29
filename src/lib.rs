use crate::Move::{RowsMatch, GroundReset};

const N_LEN: usize = 7;
const SUM: u8 = 13;

#[derive(Clone)]
pub struct Pyramid<> {
    rows: Vec<Vec<u8>>,
    ground: Vec<(u8, bool)>,
    ground_index: usize,
    reset_count: u8,
    reset_limit: u8,
}

#[derive(Clone, Debug)]
pub enum Move {
    GroundPass(u8),
    GroundMatch {
        a: u8,
        b: u8,
    },
    GroundOne,
    // one card equals to sum
    GroundReset,
    RowsMatch {
        index_a: usize,
        a: u8,
        index_b: usize,
        b: u8,
    },
    RowOne(usize),
    // one card equals to sum
    RowsGroundMatch {
        ground_b: u8,
        index_a: usize,
        a: u8,
    },
}

impl Pyramid {
    pub fn new(reset_limit: u8) -> Self {
        Pyramid {
            rows: vec![],
            ground: vec![],
            ground_index: 0,
            reset_count: 0,
            reset_limit,
        }
    }

    pub fn add_one_row(mut self, card: &[u8]) -> Self {
        self.rows.push(card.to_vec());
        self
    }

    pub fn add_grounds(mut self, ground: &[u8]) -> Self {
        ground.into_iter().for_each(|a| self.ground.push((*a, true)));
        self
    }

    pub fn last_can_move(&self, row_index: usize) -> bool {
        if self.rows[row_index].len() == 0 {
            return false;
        }

        if row_index == N_LEN - 1 {
            return true;
        }

        self.rows[row_index].len() > self.rows[row_index + 1].len()
    }
}

pub fn solve(p: &mut Pyramid, moves: &mut Vec<Move>) -> bool {
    if p.rows[0].len() == 0 {
        // solved!
        return true;
    }

    for i in 0..N_LEN {
        if !p.last_can_move(i) {
            continue;
        }

        if let Some(li) = p.rows[i].pop() {
            if li == SUM {
                moves.push(Move::RowOne(i));
                if solve(&mut p.clone(), moves) {
                    return true;
                }
                moves.pop();
            } else {
                for j in i + 1..N_LEN {
                    if !p.last_can_move(j) {
                        continue;
                    }

                    if let Some(lj) = p.rows[j].pop() {
                        if li + lj == SUM {
                            moves.push(RowsMatch {
                                index_a: i,
                                a: li,
                                index_b: j,
                                b: lj,
                            });
                            if solve(&mut p.clone(), moves) {
                                return true;
                            }
                            moves.pop();
                        }

                        p.rows[j].push(lj);
                    }
                }
            }

            p.rows[i].push(li);
        }
    }

    let move_backup = moves.len();
    let old_index = p.ground_index;

    loop {
        if p.ground[p.ground_index].1 {
            let lg = p.ground[p.ground_index].0;
            for r in 0..N_LEN - 1 {
                if !p.last_can_move(r) {
                    continue;
                }

                if let Some(lr) = p.rows[r].pop() {
                    if lr + lg == SUM {
                        p.ground[p.ground_index].1 = false;
                        moves.push(Move::RowsGroundMatch {
                            index_a: r,
                            a: lr,
                            ground_b: lg,
                        });
                        if solve(&mut p.clone(), moves) {
                            return true;
                        }
                        moves.pop();
                        p.ground[p.ground_index].1 = true;
                    }

                    p.rows[r].push(lr);
                }
            }
            moves.push(Move::GroundPass(lg));
        }

        p.ground_index = p.ground_index + 1;
        if p.ground_index == p.ground.len() {
            p.ground_index = 0;
            moves.push(GroundReset);
            p.reset_count += 1;
            if p.reset_limit != 0 && p.reset_count >= p.reset_limit {
                break;
            }
        }

        if p.ground_index == old_index {
            break;
        }
    }

    moves.truncate(move_backup);
    false
}


#[cfg(test)]
mod tests {
    use crate::{Pyramid, solve};

    #[macro_export]
    macro_rules! solve_p {
        ( $p:expr ) =>{
            let mut moves = vec![];
            println!("==========================================================");
            if solve(&mut $p, &mut moves)
            {
                moves.iter().for_each(|x| println!("{:?}", x));
            } else {
                println!("No solution!");
            }
        }
    }
    // #[test]
    // fn test_grandmaster() {
    //     let mut p = Pyramid::new()
    //         .add_one_row(&[12, 4, 12, 4, 9, 9, 8])
    //         .add_one_row(&[1, 1, 13, 5, 7, 13])
    //         .add_one_row(&[7, 7, 2, 10, 5])
    //         .add_one_row(&[8, 10, 10, 12])
    //         .add_one_row(&[6, 9, 6])
    //         .add_one_row(&[9, 8])
    //         .add_one_row(&[6])
    //         .add_grounds(&[6, 3, 11, 2, 8, 11, 11, 5, 2, 7, 3, 13, 3, 5, 11, 13, 4, 12, 1, 4, 10, 3, 2, 1]);
    //
    //     let mut moves = vec![];
    //     if solve(&mut p, &mut moves)
    //     {
    //         moves.iter().for_each(|x| println!("{:?}", x));
    //     } else {
    //         println!("No solution!");
    //     }
    // }

    #[test]
    fn test_grandmaster_2() {
        let mut p = Pyramid::new(3)
            .add_one_row(&[1, 9, 7, 8, 6, 9, 2])
            .add_one_row(&[7, 8, 5, 13, 5, 10])
            .add_one_row(&[12, 13, 3, 12, 12])
            .add_one_row(&[8, 7, 5, 7])
            .add_one_row(&[2, 3, 3])
            .add_one_row(&[6, 2])
            .add_one_row(&[4])
            .add_grounds(&[4, 10, 9, 4, 10, 11, 1, 6, 4, 6, 1, 13, 10, 11, 3, 1, 8, 9, 13, 5, 11, 12, 2, 11]);

        solve_p!(p);
    }

    #[test]
    fn test_grandmaster_3() {
        let mut p = Pyramid::new(3)
            .add_one_row(&[10, 10, 12, 10, 7, 3, 1])
            .add_one_row(&[1, 4, 13, 2, 12, 1])
            .add_one_row(&[3, 7, 12, 9, 6])
            .add_one_row(&[13, 10, 8, 2])
            .add_one_row(&[7, 11, 5])
            .add_one_row(&[3, 4])
            .add_one_row(&[2])
            .add_grounds(&[9, 2, 11, 1, 5, 4, 6, 8, 11, 8, 6, 12, 13, 7, 9, 9, 8, 13, 11, 6, 3, 5, 5, 4]);

        solve_p!(p);
    }

    #[test]
    fn test_grandmaster_4() {
        let mut p = Pyramid::new(3)
            .add_one_row(&[4, 10, 5, 5, 3, 12, 9])
            .add_one_row(&[8, 13, 13, 12, 10, 4])
            .add_one_row(&[7, 7, 3, 9, 11])
            .add_one_row(&[4, 11, 1, 6])
            .add_one_row(&[9, 3, 1])
            .add_one_row(&[10, 6])
            .add_one_row(&[2])
            .add_grounds(&[10, 1, 12, 2, 5, 8, 6, 11, 8, 11, 3, 13, 8, 2, 1, 4, 12, 5, 9, 6, 7, 7, 2, 13]);

        solve_p!(p);
    }
}
