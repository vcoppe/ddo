use std::hash::{Hash, Hasher};
use std::ops::Not;

use bitset_fixed::BitSet;

use ddo::common::{Decision, Domain, Variable, VarSet, BitSetIter};
use ddo::abstraction::dp::Problem;
use std::cmp::Reverse;

#[derive(Debug, Clone)]
pub struct State {
    pub free : BitSet,
    pub cut  : isize,
    pub cuts : Vec<isize>
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.free == other.free
    }
}
impl Eq for State {}
impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.free.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct Srflp {
    pub g : Vec<Vec<isize>>,
    pub l : Vec<isize>
}
impl Srflp {
    pub fn new(g : Vec<Vec<isize>>, l : Vec<isize>) -> Srflp {
        Srflp { g, l }
    }
}
impl Problem<State> for Srflp {
    fn nb_vars(&self) -> usize {
        self.g.len()
    }

    fn initial_state(&self) -> State {
        State {
            free: BitSet::new(self.nb_vars()).not(),
            cut: 0,
            cuts: vec![0; self.nb_vars()]
        }
    }

    fn initial_value(&self) -> isize {
        0
    }

    fn domain_of<'a>(&self, state: &'a State, _var: Variable) -> Domain<'a> {
        Domain::from(&state.free)
    }

    fn transition(&self, state: &State, vars: &VarSet, d: Decision) -> State {
        let i = d.value as usize;

        let mut result = state.clone();

        result.free.set(i, false);
        result.cuts[i] = 0;

        if state.free.count_ones() == 1 + vars.len() as u32 {
            result.cut -= state.cuts[i];
            for j in BitSetIter::new(&state.free) {
                result.cut += self.g[i][j];
                result.cuts[j] += self.g[i][j];
            }
        } else { // relaxed state
            let mut cuts = vec![];
            for j in BitSetIter::new(&state.free) {
                result.cuts[j] += self.g[i][j];
                cuts.push(result.cuts[j]);
            }
            // compute cut as the sum of the smallest cut values for remaining spots
            cuts.sort_unstable();
            result.cut = 0;
            for j in 0..vars.len() {
                result.cut += cuts[j];
            }
        }

        result
    }

    fn transition_cost(&self, state: &State, _vars: &VarSet, d: Decision) -> isize {
        let i = d.value as usize;

        - self.l[i] * (state.cut - state.cuts[i]).max(0)
    }
}