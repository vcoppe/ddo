use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, Duration};

use structopt::StructOpt;

use ddo::abstraction::solver::Solver;
use ddo::implementation::mdd::config::config_builder;
use ddo::implementation::mdd::aggressively_bounded::AggressivelyBoundedMDD;
use ddo::implementation::solver::parallel::ParallelSolver;
use ddo::implementation::frontier::NoForgetFrontier;
use ddo::implementation::heuristics::{TimeBudget, FixedWidth};

use crate::graph::Graph;
use crate::model::Minla;
use crate::relax::MinlaRelax;
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
    let relax = MinlaRelax::new(&problem);
    let cfg = config_builder(&problem, relax)
        .with_cutoff(TimeBudget::new(Duration::from_secs(time)))
        .with_max_width(FixedWidth(opt.width.unwrap_or(problem.nb_vars())))
        .build();
    let mdd = AggressivelyBoundedMDD::from(cfg);
    let mut solver  = ParallelSolver::customized(mdd, 2, num_cpus::get())
        .with_frontier(NoForgetFrontier::default());

    let start = SystemTime::now();
    let opt = solver.maximize().best_value.unwrap_or(isize::min_value());
    let end = SystemTime::now();

    println!("Best {} computed in {:?}", opt, end.duration_since(start).unwrap());
}

fn read_file(fname: &str) -> Result<Minla, io::Error> {
    if fname.contains("gra") {
        read_gra(fname)
    } else if fname.contains("dimacs") {
        read_dimacs(fname)
    } else if fname.contains("mtx") {
        read_mtx(fname)
    } else {
        read_gra(fname)
    }
}

fn read_gra(fname: &str) -> Result<Minla, io::Error> {
    let file = File::open(fname).expect("File not found.");
    let buffered = BufReader::new(file);

    let mut numbers = Vec::new();
    for line in buffered.lines() {
        let line = line?;
        let iter = line.trim().split_whitespace();
        for x in iter {
            let x = x.parse::<i32>().unwrap();
            if x >= 0 {
                numbers.push(x as usize)
            }
        }
    }

    let n = numbers[0];
    let mut g = vec![vec![0; n]; n];

    let mut cumul= 2+n;
    for i in 0..n {
        for j in cumul..(cumul+numbers[2+i]) {
            g[i][numbers[j]] = 1;
            g[numbers[j]][i] = 1;
        }
        cumul += numbers[2+i];
    }

    Ok(Minla::new(g))
}

fn read_dimacs(fname: &str) -> Result<Minla, io::Error> {
    let file = File::open(fname).expect("File not found.");
    let graph = Graph::from(file);

    let n = graph.nb_vertices;
    let mut g = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            g[i][j] = graph[(i,j)];
        }
    }

    Ok(Minla::new(g))
}

fn read_mtx(fname: &str) -> Result<Minla, io::Error> {
    let file = File::open(fname).expect("File not found.");
    let buffered = BufReader::new(file);
    let mut n = 0;
    let mut g = vec![];

    for line in buffered.lines() {
        let line = line?;

        if line.starts_with('%') {
            continue;
        }

        let data: Vec<f32> = line.trim().split_whitespace().map(|s| s.parse::<f32>().unwrap()).collect();

        if n == 0 {
            n = data[0] as usize;
            g = vec![vec![0; n]; n];
        } else {
            let i = (data[0] as usize)-1;
            let j = (data[1] as usize)-1;
            g[i][j] = 1;
            g[j][i] = 1;
        }
    }

    Ok(Minla::new(g))
}
