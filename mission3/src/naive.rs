//! The naive AI
//! This ai will just simulate the game one turn ahead and choose the move wich looked the best.
//! This is equivalent to a min max algorithm of depth 1.
//!
use game::Game;
use unit::Action;
use ai::AI;
use evaluator::Evaluator;
/// The naive ai struct.
pub struct NaiveAI {
    evaluator: Box<Evaluator>,
}
impl NaiveAI {
    /// Creates a new naive ai.
    pub fn new(eval: Box<Evaluator>) -> Self {
        NaiveAI { evaluator: eval }
    }
    /// Evaluates the game's state.
    pub fn eval(&self, game: &Game, player: usize) -> i32 {
        self.evaluator.eval(game, player)
    }
}
impl AI for NaiveAI {
    /// Simulates all the possible moves, evaluate the game state and choose the move
    /// with the max score.
    fn play(&mut self, game: &Game, player: usize) -> Action {
        let moves = game.moves(player);
        moves
            .iter()
            .map(|mv| {
                let mut g = game.clone();
                g.apply_move(mv);
                (mv, self.eval(&g, player))
            })
            .max_by_key(|x| x.1)
            .unwrap()
            .0
            .clone()
    }
}
