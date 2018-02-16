/// The random "ai"
/// Will play random moves everytime.
/// Random moves include killing its own units.
use game::Game;
use unit::Action;
use ai::AI;
use rand::{XorShiftRng, Rng};
/// THe random ai struct.
pub struct RandomAI {
    /// The random generator.
    rand: XorShiftRng,
}
impl RandomAI {
    /// Creates a new random ai.
    pub fn new() -> Self {
        RandomAI { rand: XorShiftRng::new_unseeded() }
    }
}
impl AI for RandomAI {
    /// Selects a random move among all moves disponibles.
    fn play(&mut self, game: &Game, player: usize) -> Action {
        let moves = game.moves(player);
        self.rand.choose(&moves).unwrap().clone()
    }
}
