use ddo::abstraction::dp::{Relaxation, Problem};
use ddo::common::Decision;

use crate::model::{Srflp, State};

use bitset_fixed::BitSet;
use std::cmp::Reverse;

#[derive(Debug, Clone)]
pub struct SrflpRelax<'a> {
    pb: &'a Srflp,
    default_relaxed_state: State
}

impl <'a> SrflpRelax<'a> {
    pub fn new(pb : &'a Srflp) -> SrflpRelax<'a> {
        SrflpRelax {
            pb,
            default_relaxed_state: State {
                free: BitSet::new(0),
                cut: 0,
                cuts: vec![0; pb.nb_vars()]
            }
        }
    }

    fn ub(&self, state: &State) -> isize {
        let n = state.free.count_ones() as usize;
        let mut cuts = state.cuts.clone();

        cuts.sort_unstable_by_key(|&x| Reverse(x));

        let mut edge_lb = 0;
        let mut cut_lb = 0;
        let mut cumul_l = 0;
        let mut edge_idx = 0;
        let mut length_idx = 0;
        for i in 0..n {
            for _ in 0..(n-i-1) {
                while !state.free[self.pb.edges[edge_idx].1] || !state.free[self.pb.edges[edge_idx].2] {
                    edge_idx += 1;
                }
                edge_lb += cumul_l * self.pb.edges[edge_idx].0;
                edge_idx += 1;
            }

            cut_lb += cumul_l * cuts[i];

            while !state.free[self.pb.lengths[length_idx].1] {
                length_idx += 1;
            }
            cumul_l += self.pb.lengths[length_idx].0;
            length_idx += 1;
        }

        - edge_lb - cut_lb
    }
}
impl <'a> Relaxation<State> for SrflpRelax<'a> {
    fn merge_states(&self, _states: &mut dyn Iterator<Item=&State>) -> State {
        self.default_relaxed_state.clone()
    }

    fn relax_edge(&self, _src: &State, dst: &State, _relaxed: &State, _decision: Decision, cost: isize) -> isize {
        cost + self.ub(&dst)
    }

    fn estimate(&self, state : &State) -> isize {
        self.ub(&state)
    }

    fn default_relaxed_state(&self) -> State {
        self.default_relaxed_state.clone()
    }
}