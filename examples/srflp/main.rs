use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, Duration};

use structopt::StructOpt;
use regex::Regex;

use ddo::abstraction::solver::Solver;
use ddo::implementation::mdd::config::config_builder;
use ddo::implementation::mdd::aggressively_bounded::AggressivelyBoundedMDD;
use ddo::implementation::frontier::NoDupFrontier;
use ddo::implementation::heuristics::{TimeBudget, FixedWidth};
use ddo::implementation::solver::parallel::ParallelSolver;

use crate::model::Srflp;
use crate::relax::SrflpRelax;
use ddo::abstraction::dp::Problem;

mod model;
mod relax;
mod graph;

#[derive(StructOpt)]
struct Opt {
    /// Path to the instance (*.gra | *.dimacs)
    fname: String,
    /// If specified, the maximum time allowed
    #[structopt(short, long)]
    time: Option<u64>,
    /// If specified, the maximum width allowed
    #[structopt(short, long)]
    width: Option<usize>
}

fn main() {
    let opt = Opt::from_args();

    let time = opt.time.unwrap_or(u64::max_value());
    let problem = read_file(&opt.fname).unwrap();
    let relax = SrflpRelax::new(&problem);
    let cfg = config_builder(&problem, relax)
        .with_cutoff(TimeBudget::new(Duration::from_secs(time)))
        .with_max_width(FixedWidth(opt.width.unwrap_or(problem.nb_vars())))
        .build();

    let mdd = AggressivelyBoundedMDD::from(cfg);
    let mut solver  = ParallelSolver::customized(mdd, 2, num_cpus::get())
        .with_frontier(NoDupFrontier::default());

    let start = SystemTime::now();
    let mut opt = solver.maximize().best_value.unwrap_or(isize::min_value()) as f32;
    let end = SystemTime::now();

    for i in 0..problem.nb_vars() {
        for j in 0..problem.nb_vars() {
            opt -= 0.5 * ((problem.g[i][j] * problem.l[i]) as f32);
        }
    }

    println!("Best {} computed in {:?}", opt, end.duration_since(start).unwrap());
}

fn read_file(fname: &str) -> Result<Srflp, io::Error> {
    read(fname)
}

fn read(fname: &str) -> Result<Srflp, io::Error> {
    let file = File::open(fname).expect("File not found.");
    let buffered = BufReader::new(file);
    let mut n = 0;
    let mut l = vec![];
    let mut g = vec![];

    let mut row = 0;
    let re = Regex::new(r"[\D+]").unwrap();

    for line in buffered.lines() {
        let li = line?;
        let formatted_line = re.replace_all(li.trim(), ",");

        let data: Vec<isize> = formatted_line.split(',').filter(|s| s.len() > 0).map(|s| s.parse::<isize>().unwrap()).collect();

        if n == 0 {
            n = data[0] as usize;
            g = vec![vec![]; n];
        } else if l.len() == 0 {
            l = data;
        } else if data.len() == n {
            g[row] = data;
            row += 1;
        }
    }

    Ok(Srflp::new(g, l))
}
