use crate::core::abstraction::heuristics::{WidthHeuristic, VariableHeuristic};
use crate::core::abstraction::mdd::{MDD, Node};
use std::hash::Hash;
use crate::core::abstraction::dp::{VarSet, Variable};

pub struct FixedWidth(pub usize);

impl <T, N> WidthHeuristic<T, N> for FixedWidth
    where T : Clone + Hash + Eq,
          N : Node<T, N> {
    fn max_width(&self, _dd: &dyn MDD<T, N>) -> usize {
        self.0
    }
}

pub struct NaturalOrder;
impl NaturalOrder {
    pub fn new() -> NaturalOrder {
        NaturalOrder{}
    }
}
impl <T, N> VariableHeuristic<T, N> for NaturalOrder
    where T : Clone + Hash + Eq,
          N : Node<T, N> {

    fn next_var(&self, _dd: &dyn MDD<T, N>, vars: &VarSet) -> Option<Variable> {
        for x in vars.iter() {
            return Some(x)
        }
        None
    }
}