//! Implementation of the min max algorithm.
use std::fmt;
use std::cmp;
use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

impl Strategy for MinMax {

    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let depth = self.0;
        let mut mvmt = None;
        //maximize
        if depth % 2 == 1 {
            println!("Maximize");
            let mut val = -100;
            for mov in state.movements() {
                let cur_val = find_next_move(depth - 1, false, &Configuration::play(state, &mov));
                if cur_val > val {
                    mvmt = Some(mov);
                    val = cur_val;
                }
            }
        }
        //minimize
        else {
            println!("Minimize");
            let mut val = 100;
            for mov in state.movements() {
                let cur_val = find_next_move(depth - 1, true, &Configuration::play(state, &mov));
                if cur_val < val {
                    mvmt = Some(mov);
                    val = cur_val;
                }
            }
        }
        return mvmt;
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}

fn find_next_move(depth : u8, maximize : bool, state: &Configuration) -> i8 {
    if depth == 0 {
        //println!("reach 0 depth");
        //println!("{:?}", Configuration::value(state));
        return Configuration::value(state)
    }
    //maximizing player
    if maximize {
        let mut val = -100;
        for mov in state.movements() {
            val = cmp::max(find_next_move(depth - 1, false,  &Configuration::play(state, &mov)) , val);
        }
        //println!("Passé par le maximizing player");
        return val;
    }
    // minimizing player
    else {
        let mut val = 100;
        for mov in state.movements() {
            val = cmp::min(find_next_move(depth - 1, true, &Configuration::play(state, &mov)), val);
        }
        //println!("Passé par le minimizing player");
        return val;
    }
}
