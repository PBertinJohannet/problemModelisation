#[deny(missing_docs)]
/// The crate for mission3
/// Contains the logic for the game and a few ais.
/// A minimax AI in the minmax.rs file.
/// A naive AI (evaluating the moves one turn ahead).
/// A random AI (Not actually an ai).
/// A montecarlo tree search AI.
/// The crate also contain an Arena struct in wich we can test two ai against each other on a lot of
/// games.

extern crate rand;
pub mod game;
pub mod ai;
pub mod unit;
pub mod randomai;
pub mod naive;
pub mod minmax;
pub mod montecarlo;
pub mod evaluator;
pub mod comparator;
