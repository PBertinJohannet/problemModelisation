/// Module for the basic AI behaviours.
/// This contains the trait for ais and anarena to test them.
/// To create an ai just implements the *AI* trait and launch it in the arena.
use game::Game;
use unit::Action;
use rand::{XorShiftRng};


/// The AI trait.
/// An AI only have to return an action given a game and a player.
pub trait AI {
    /// Returns an action, given a player and the state of the game.
    fn play(&mut self, game: &Game, player: usize) -> Action;
}

/// The arena struct, used to test ais.
/// An arena contains a game configuration and make two ais play against each other in a number
/// of generated games.
pub struct Arena {
    /// The size of the game.
    /// The game will always take place in a square grid.
    game_size: usize,
    /// The number of game that will be played in the arena.
    game_number: usize,
    /// The number of units per players.
    game_units: usize,
    /// The verbose attribute. when set to true. the arena will print the result of each separate game.
    pub verbose: bool,
    /// The maximum number of rounds until the game ends.
    max_turns: usize,
    /// Progression of the difficulty.
    /// Every *step* games the difference of units between the players increase
    progression_step: usize,
    /// The number of units that we add/remove every progression step.
    progression_units: usize,
}

impl Arena {
    /// Creates a new arena with the given parameters.
    pub fn new(size: usize, number: usize, units: usize, verbose: bool, max_turns: usize) -> Self {
        Arena {
            game_size: size,
            game_number: number,
            game_units: units,
            verbose: verbose,
            max_turns: max_turns,
            progression_step: 1,
            progression_units: 0,
        }
    }
    /// Sets the progression of the unbalance between the players.
    /// every *step* games the difference in units decrease by *units*
    pub fn progression(&mut self, step: usize, units: usize) -> &mut Self {
        self.progression_units = units;
        self.progression_step = step;
        self
    }

    /// Runs a fight with the two given ais.
    /// Takes a mutable reference to two structs implementing the *AI* trait and make them play
    /// Then returns the number of wins for each one.
    ///
    /// For each randomly generated game. THe ais will play two times.
    /// Each time, a different ai will make the first move.
    ///
    /// If the parameters for progression were modified
    /// Every *step* games a difference in the number of units of each player will be created.
    pub fn fight(&self, ai: &mut AI, ai2: &mut AI) -> (usize, usize) {
        let mut my_rand = XorShiftRng::new_unseeded();
        let mut wons = [0, 0];
        let mut advantages = [0, 0];
        for i in 1..self.game_number +1 {
            let mut game = Game::new_random(
                self.game_size,
                self.game_size,
                self.game_units + advantages[1],
                &mut my_rand,
            );
            game.remove_unit_for_player(0, advantages[0]);
            if i % self.progression_step == 0 {
                if self.game_units - advantages[0] > 1 {
                    advantages[0] += 1;
                }
                if self.game_units + advantages[1] < self.game_size * self.game_size / 3 {
                    advantages[1] += 1;
                }
            }
            let res = self.rounds(&game, ai, ai2);
            wons[0] += res.0;
            wons[1] += res.1;
            println!(" {} ---------------------------------------------", i);
        }
        println!("ai 1 : {} wins.", wons[0]);
        println!("ai 2 : {} wins.", wons[1]);
        (wons[0], wons[1])
    }

    /// makes two ais play on a game set. Each ai takes each side one time.
    /// returns a tuple of the wins of the first and second ai
    pub fn rounds(&self, game: &Game, ai: &mut AI, ai2: &mut AI) -> (usize, usize) {
        let mut wons = [0, 0];
        let res = Self::resolve(&mut game.clone(), &mut [ai, ai2], self.max_turns);
        wons[res] += 1;
        if self.verbose {
            println!("ai : {} won", res + 1);
        }
        let res = 1 - Self::resolve(&mut game.clone(), &mut [ai2, ai], self.max_turns);
        wons[res] += 1;
        if self.verbose {
            println!("ai : {} won", res + 1);
        }
        (wons[0], wons[1])
    }

    /// Given a game's starting state, and two ais in an array, make them play and returns the id of
    /// the winner.
    /// The ai with index 0 in the array will start first.
    ///
    /// If no unit dies for *nb_rounds* turns then
    ///
    /// returns the id of the player with the most unit still alive
    /// If the units are the same returns the id of the player who started playing.
    pub fn resolve(game: &mut Game, ais: &mut [&mut AI], nb_rounds: usize) -> usize {
        let mut rounds_since_last_death = 0;
        let mut alive_units = game.player(0).len() + game.player(1).len();
        'a: loop {
            for i in 0..2 {
                if game.player(i).len() > 0 && game.moves(i).len() > 0 {
                    let mv = &ais[i].play(&game, i);
                    game.apply_move(mv);
                } else {
                    return 1 - i;
                }
            }
            let new_unit_count = game.player(0).len() + game.player(1).len();
            if new_unit_count != alive_units {
                rounds_since_last_death += 0;
                alive_units = new_unit_count;
            }
            rounds_since_last_death += 1;
            if rounds_since_last_death > nb_rounds {
                break;
            }
        }
        (game.player(1).len() > game.player(0).len()) as usize
    }
}
