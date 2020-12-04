use ddo::abstraction::dp::{Relaxation, Problem};
use ddo::common::{Decision, BitSetIter};

use crate::model::{Srflp, State};

use bitset_fixed::BitSet;
use std::cmp::Reverse;
use std::ops::BitOrAssign;

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
}
impl <'a> Relaxation<State> for SrflpRelax<'a> {
    fn merge_states(&self, states: &mut dyn Iterator<Item=&State>) -> State {
        let mut free = BitSet::new(self.pb.nb_vars());
        let mut cut = isize::max_value();
        let mut cuts = vec![isize::max_value(); self.pb.nb_vars()];

        for state in states {
            free.bitor_assign(&state.free);
            cut = cut.min(state.cut);
            for i in BitSetIter::new(&state.free) {
                cuts[i] = cuts[i].min(state.cuts[i]);
            }
        }

        for i in 0..self.pb.nb_vars() {
            if !free[i] {
                cuts[i] = 0;
            }
        }

        State { free, cut, cuts }
    }

    fn relax_edge(&self, _src: &State, dst: &State, _relaxed: &State, _decision: Decision, cost: isize) -> isize {
        cost // + self.ub(&dst)
    }

    fn estimate(&self, state : &State) -> isize {
        0 // self.ub(&state)
    }

    fn default_relaxed_state(&self) -> State {
        self.default_relaxed_state.clone()
    }
}