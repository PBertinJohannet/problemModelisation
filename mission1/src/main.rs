#[deny(missing_docs)]
/// The crate for mission1
/// Uses a star to find the way to ship all the containers in less possible moves.
extern crate graphs;
extern crate rand;
extern crate time;

use time::PreciseTime;

use graphs::graphs::*;

mod shippment;
mod place;
mod heuristic;
use shippment::{Shippment, print_sol};
use rand::XorShiftRng;


const MAX_PLACE : usize = 5;
const DEF_NUM_PLACE : usize = 9;

/// Launching it will print the results and the number of iterations to get these results
///
fn main() {
    let mut my_rand = XorShiftRng::new_unseeded();

    // Trying with the course example.

    println!("trying to solve the instance of the problem given in the course using dfs");
    let ship = Shippment::new_course();
    let start = PreciseTime::now();
    let sol = dfs(ship).unwrap();
    let end = PreciseTime::now();
    let diff = start.to(end).num_milliseconds();
    println!(" found a solution in : {} ms", diff);
    println!("the solution found is : \n\n");
    print_sol(sol);
    println!("\n\n");

    println!("\n\nNow trying problems of increasing sizes with a star\n\n");


    for i in 0..10 {
        println!("trying an instance of the problem with \n {} emplacement on the boat\n\
        {} stacks of {} containers on the harbour.", (i/4+1)*i, i, i);
        let ship = Shippment::new_random((i/4+1)*i, i, i, &mut my_rand);
        let start = PreciseTime::now();
        let sol = AStar::new(ship).solve().unwrap();
        let end = PreciseTime::now();
        let diff = start.to(end).num_milliseconds();
        println!(" found a solution in : {} ms", diff);
        println!("the solution found is : \n\n");
        print_sol(sol);
        println!("\n\n");
        solve_for(i, &mut my_rand);
        println!("\n");
    }
}

/// Solves some random problems of the given size.
pub fn solve_for(i : usize, my_rand : &mut XorShiftRng){
    println!("\n\nsolving 500 problems of size {}", i);
    let start = PreciseTime::now();
    for _ in 0..500 {
        let ship = Shippment::new_random((i + 1) * i, i, i, my_rand);
        let _sol = AStar::new(ship).solve().unwrap_or(vec![]);
    }
    let end = PreciseTime::now();
    let diff = start.to(end).num_milliseconds();
    println!(" found all 500 solutions in : {} ms\n", diff);
}