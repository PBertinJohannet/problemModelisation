#[deny(missing_docs)]
/// The crate for mission2
extern crate rand;
extern crate graphs;
extern crate itertools;

extern crate time;
use time::PreciseTime;

use graphs::graphs::AStar;

mod harbours;
use harbours::{Harbour, print_sol};
use rand::XorShiftRng;
use rand::Rng;



/// This will try to find the best solution for i boats
/// i ranging from 2 to 50.
/// Usually it starts getting too long at 13 boats.
pub fn main() {
    // each ship is the time needed in hours to get to the other side.

    println!("trying the course example. unit => 1/4h");
    let ships = vec![3, 6, 15, 24];
    simple_sol(&ships);
    let start = PreciseTime::now();
    let sol = AStar::new(Harbour::new(ships.clone())).solve().unwrap_or_else(||panic!("fail"));
    let end = PreciseTime::now();
    let diff = start.to(end).num_milliseconds();
    println!(" found a solution in : {} ms", diff);
    println!("the solution found is : \n\n");
    print_sol(sol);
    println!("\n\n");


    let mut my_rand = XorShiftRng::new_unseeded();
    for i in 4..15 {
        println!("trying to find a solution for {} ships", i);
        let  ships = my_rand.gen_iter().take(i).map(|i : i32 | (i.abs() % 10 +1) as i32 ).collect::<Vec<i32>>();
        println!("the ships are : {:?}", ships);
        let start = PreciseTime::now();
        let sol = AStar::new(Harbour::new(ships.clone())).solve();
        let end = PreciseTime::now();
        let diff = start.to(end).num_milliseconds();
        println!(" found a solution in : {} ms", diff);
        println!("the solution found is : \n\n");
        print_sol(sol.unwrap());
        println!("\n\n")
    }
}


/// This is the solution in linearithmic time explained at the end of the report.
pub fn simple_sol(ships: &Vec<i32>) -> Vec<(i32, i32)>{
    let mut sol = vec![];
    let mut s_ships = ships.clone();
    s_ships.sort();
    sol.push((s_ships[0], s_ships[1]));
    //println!("move {} and {} to end and bring {} back.", s_ships[0], s_ships[1], s_ships[0]);
    while !s_ships.is_empty() {
        if s_ships.len() > 2 {
            sol.push((s_ships[0], s_ships[2]));
            //println!("move {} and {} to end", s_ships[0], s_ships[2]);
            break;
        }
        let bigger = s_ships.pop().unwrap();
        if s_ships[1]*2-s_ships[1] < *s_ships.last().unwrap(){
            sol.push((*s_ships.last().unwrap(), bigger));
            sol.push((s_ships[0], s_ships[1]));
            //println!("move {} and {} to end and bring {} back.", s_ships.last().unwrap(), bigger, s_ships[1]);
            //println!("move {} and {} to end and bring {} back.", s_ships[0], s_ships[1], s_ships[0]);
            s_ships.pop();
        } else {
            sol.push((s_ships[0], bigger));
            //println!("move {} and {} to end and bring {} back.", s_ships[0], bigger, s_ships[0]);
        }
    }
    sol
}