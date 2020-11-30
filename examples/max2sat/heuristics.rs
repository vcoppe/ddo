// Copyright 2020 Xavier Gillard
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::cmp::Ordering;

use ddo::abstraction::heuristics::{NodeSelectionHeuristic, SelectableNode, VariableHeuristic};
use ddo::common::{Variable, VarSet};

use crate::model::{Max2Sat, State};

#[derive(Debug, Clone)]
pub struct Max2SatOrder<'a> {
    problem: &'a Max2Sat
}

impl <'a> Max2SatOrder<'a> {
    pub fn new(problem: &'a Max2Sat) -> Max2SatOrder<'a> {
        Max2SatOrder{problem}
    }
}

impl VariableHeuristic<State> for Max2SatOrder<'_> {
    fn next_var(&self,
                free_vars: &VarSet,
                _: &mut dyn Iterator<Item=&State>,
                _: &mut dyn Iterator<Item=&State>) -> Option<Variable>
    {
        let mut var = None;
        let mut wt  = isize::min_value();

        for v in free_vars.iter() {
            let v_wt = self.problem.sum_of_clause_weights[v.0];
            if v_wt > wt {
                var = Some(v);
                wt  = v_wt;
            }
        }

        var
    }
}

#[derive(Debug, Clone)]
pub struct MinRank;
impl NodeSelectionHeuristic<State> for MinRank {
    fn compare(&self, x: &dyn SelectableNode<State>, y: &dyn SelectableNode<State>) -> Ordering {
        let xrank = x.value() + x.state().rank();
        let yrank = y.value() + y.state().rank();
        xrank.cmp(&yrank)
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::path::PathBuf;

    use ddo::abstraction::dp::Problem;
    use ddo::abstraction::heuristics::VariableHeuristic;

    use crate::heuristics::Max2SatOrder;
    use crate::model::Max2Sat;

    #[test]
    fn variable_ordering() {
        let problem = instance("frb10-6-1.wcnf");
        let order   = Max2SatOrder::new(&problem);
        let mut vars= problem.all_vars();

        let mut actual= vec![];
        for _ in 0..problem.nb_vars() {
            let empty = vec![];
            let v = order.next_var(&vars, &mut empty.iter(), &mut empty.iter()).unwrap();
            vars.remove(v);
            actual.push(v.0);
        }

        let expected = vec![
            26, 24, 28, 25, 27, 32, 43, 44, 45, 42, 47, 52, 19, 34, 11, 22, 46,
            49, 50,  4,  8, 16, 53,  5,  9, 18, 23, 48,  0, 20, 59,  1, 35, 17,
            31, 39, 54, 57,  2,  3, 14, 15, 30, 38, 55,  6,  7, 10, 12, 29, 33,
            37, 51, 56, 58, 13, 21, 36, 40, 41
        ];
        assert_eq!(actual, expected);
    }

    fn locate(id: &str) -> PathBuf {
        PathBuf::new()
            .join(env!("CARGO_MANIFEST_DIR"))
            .join("examples/tests/resources/max2sat/")
            .join(id)
    }

    fn instance(id: &str) -> Max2Sat {
        let location = locate(id);
        File::open(location).expect("File not found").into()
    }
}