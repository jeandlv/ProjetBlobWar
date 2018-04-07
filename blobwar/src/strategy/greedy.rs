//! Dumb greedy algorithm.
use std::fmt;
use super::Strategy;
use configuration::{Configuration, Movement};

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let mut mvmt = None;
        let mut val = -100;  
        for mov in state.movements(){
            let cur_val = Configuration::value(&Configuration::play(state, &mov)); 
            if cur_val > val {
                mvmt = Some(mov);
                val = cur_val;
            } 
        }
        return mvmt;
    }
}
