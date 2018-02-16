#![deny(missing_docs)]

//! The crate for mission3
//! Contains the logic for the game and a few ais.
//! A minimax AI in the minmax.rs file.
//! A naive AI (evaluating the moves one turn ahead).
//! A random AI (Not actually an ai).
//! A monte carlo tree search AI.
//! The crate also contain an Arena struct in wich we can test two ai against each other on a lot of
//! games.
extern crate rand;
mod game;
mod ai;
mod unit;
mod randomai;
mod naive;
mod minmax;
mod montecarlo;
mod evaluator;
mod comparator;

use ai::*;
use naive::NaiveAI;
use minmax::NegaMaxAI;
use montecarlo::MonteCarloAI;
use comparator::{get_all_naives, unit_eval_double};

/// The main function.
/// Here we launch some fight between ais to compare them.
pub fn main() {
    let arena = &mut Arena::new(10, 15, 6, false, 100);
    arena.progression(4, 1); // every 3 games, favorise a player by one unit.



    println!(
        "comparison of different heuristics for the naive ai.\n\
            Everytime the ais are tested against each other in 30 games."
    );

    // here we can notice that the all combined is more efficient vs bad ais
    // but that the double unit counting is better overall.
    for ai in get_all_naives().iter_mut() {
        println!("\n\ncomparison of : {}\n", ai.name());
        for ai_enn in get_all_naives().iter_mut() {
            println!("comparison of {} vs {}", ai.name(), ai_enn.name());
            arena.fight(ai.ai_mut(), ai_enn.ai_mut());
        }
    }


    println!(
        "\n\ncomparison of naive with double unit counting vs the minmax ai with\
            the same heuristic and depth 2.\n\
            the ais are tested against each other in 30 games."
    );

    // the best eval was the double unit counting.
    // 12 vs 18
    arena.fight(
        &mut NaiveAI::new(unit_eval_double()),
        &mut NegaMaxAI::new(1, unit_eval_double()),
    );

    println!(
        "\n\ncomparison of naive with double unit counting vs the montecarlo tree seach ai with\
            the same heuristic.\n\
            the ais are tested against each other in 30 games."
    );

    // the best eval was the double unit counting.
    // 15 vs 15
    arena.fight(
        &mut NaiveAI::new(unit_eval_double()),
        &mut MonteCarloAI::new( unit_eval_double()),
    );

    println!(
        "\n\ncomparison of mcts with double unit counting vs the minmax ai with\
            the same heuristic and depth 2.\n\
            the ais are tested against each other in 30 games."
    );

    // the best eval was the double unit counting.
    // 13 wins for mcts vs 17 for negamax
    arena.fight(
        &mut MonteCarloAI::new(unit_eval_double()),
        &mut NegaMaxAI::new(1, unit_eval_double()),
    );
}



