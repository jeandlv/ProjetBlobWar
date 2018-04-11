//! Alpha - Beta algorithm.
use std::fmt;
use std::cmp;
use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let depth = self.0;
        let mut mvmt = None;
        let mut alpha = -100;
        let mut beta = 100;
        //maximize
        if depth % 2 == 1 {
            println!("Maximize");
            let mut val = -100;
            for mov in state.movements() {
                let cur_val = find_next_move(depth - 1, false, alpha, beta, &Configuration::play(state, &mov));
                if cur_val > val {
                    mvmt = Some(mov);
                    val = cur_val;
                }
                if val >= beta {
                    return mvmt;
                }
                alpha = cmp::max(alpha, val);
            }
        }
        //minimize
        else {
            println!("Minimize");
            let mut val = 100;
            for mov in state.movements() {
                let cur_val = find_next_move(depth - 1, true, alpha, beta, &Configuration::play(state, &mov));
                if cur_val < val {
                    mvmt = Some(mov);
                    val = cur_val;
                }
                if alpha >= val {
                    return mvmt;
                }
                beta = cmp::min(beta, val);
            }
        }
        return mvmt;
    }
}

fn find_next_move(depth : u8, maximize : bool, mut alpha : i8, mut beta : i8, state: &Configuration) -> i8 {
    if depth == 0 {
        return Configuration::value(state);
    }
    if maximize {
        let mut val = -100;
        for mov in state.movements() {
            val = cmp::max(val, find_next_move(depth-1, false, alpha, beta, &Configuration::play(state, &mov)));
            if val >= beta {
                return val;
            }
            alpha = cmp::max(alpha, val);
        }
        return val;
    }
    else {
        let mut val = 100;
        for mov in state.movements() {
            val = cmp::min(val, find_next_move(depth-1, true, alpha, beta, &Configuration::play(state, &mov)));
            if alpha >= val {
                return val;
            }
            beta = cmp::min(beta, val);
        }
        return val;
    }
}
