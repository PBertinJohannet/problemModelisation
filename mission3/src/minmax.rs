//! The minmax ai
//! This module uses a variant called negamax where we consider
//! that our score is always the opposite of the ennemy's score.
use game::Game;
use unit::Action;
use ai::AI;
use evaluator::Evaluator;

/// The negamax ai.
/// will use the minmax algorithm.
pub struct NegaMaxAI {
    /// The depth of the searched tree.
    depth: usize,
    /// The evaluator used.
    evaluator: Box<Evaluator>,
}
impl NegaMaxAI {
    /// Creates a new ai with the given evaluator.
    pub fn new(depth: usize, evaluator: Box<Evaluator>) -> Self {
        NegaMaxAI { depth, evaluator }
    }
    /// Evaluates the state of the game for the given player.
    /// The evaluation is made with the min max algorithm.
    /// But because we consider that the ennemy score is the opposite of our score.
    /// we can always take the max value and negate it before returning it.
    pub fn eval(&self, game: &Game, player: usize, depth: usize) -> i32 {
        if depth == 0 || game.player(player).len() == 0 {
            self.evaluator.eval(game, player)
        } else {
            let moves = game.moves(player);
            moves
                .iter()
                .map(|mv| {
                    let mut g = game.clone();
                    g.apply_move(mv);
                    self.eval(&g, 1 - player, depth - 1) * -1
                })
                .max()
                .unwrap()
        }
    }
}
impl AI for NegaMaxAI {
    /// Makes a choice using the min max algorithm.
    fn play(&mut self, game: &Game, player: usize) -> Action {
        let moves = game.moves(player);
        moves
            .iter()
            .map(|mv| {
                let mut g = game.clone();
                g.apply_move(mv);
                (mv, self.eval(&g, 1 - player, self.depth) * -1)
            })
            .max_by_key(|x| x.1)
            .unwrap()
            .0
            .clone()
    }
}
