use std::hash::{Hash, Hasher};
use std::ops::Not;

use bitset_fixed::BitSet;

use ddo::common::{Decision, Domain, Variable, VarSet};
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
    pub l : Vec<isize>,
    pub edges : Vec<(isize,usize,usize)>,
    pub lengths : Vec<(isize,usize)>,
}
impl Srflp {
    pub fn new(g : Vec<Vec<isize>>, l : Vec<isize>) -> Srflp {
        let mut edges = vec![];
        let mut lengths = vec![];
        for (i, is) in g.iter().enumerate() {
            for (j, js) in is.iter().enumerate() {
                if i < j {
                    edges.push((*js, i, j));
                }
            }
            lengths.push((l[i], i));
        }
        edges.sort_unstable_by_key(|&x| Reverse(x.0));
        lengths.sort_unstable();
        Srflp { g, l, edges, lengths }
    }

    fn no_vertex(&self) -> usize {
        self.nb_vars()
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
        let n = state.free.count_ones() as usize;
        if n == 0 { // relaxed node with empty free vertices intersection
            Domain::from(vec![self.no_vertex() as isize])
        } else {
            Domain::from(&state.free)
        }
    }

    fn transition(&self, state: &State, _vars: &VarSet, d: Decision) -> State {
        let i = d.value as usize;

        let mut result = state.clone();

        if i != self.no_vertex() {
            result.free.set(i as usize, false);
            result.cuts[i] = 0;

            for j in 0..self.nb_vars() {
                if state.free[j] {
                    result.cut += self.g[i][j];
                    result.cuts[j] += self.g[i][j];
                } else {
                    result.cut -= self.g[i][j];
                }
            }
        }

        result
    }

    fn transition_cost(&self, state: &State, _vars: &VarSet, d: Decision) -> isize {
        let i = d.value as usize;

        if i == self.no_vertex() {
            0
        } else {
            - self.l[i] * (state.cut - state.cuts[i])
        }
    }
}