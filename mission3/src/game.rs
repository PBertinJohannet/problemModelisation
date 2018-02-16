/// The game actualy.
use std::fmt;
use unit::{Unit, Action, Position};
use rand::{XorShiftRng};


#[derive(Clone, Hash, PartialEq, Eq, Debug)]
/// The game's struct.
/// Contains a state of the game.
pub struct Game {
    /// The width of the game's grid.
    width: usize,
    /// The height of the game's grid.
    height: usize,
    /// A two dimensional vector containing the list of units for each players.
    players: Vec<Vec<Unit>>,
}
impl Game {
    /// Creates a new game with the desired dimension and the given starting units.
    pub fn new(width: usize, height: usize, players: Vec<Vec<Unit>>) -> Self {
        Game {
            height: height,
            width: width,
            players: players,
        }
    }
    /// Creates a new random game with the given dimensions and randomly populate it with
    /// The given number of units for each player.
    /// The units generated will be symmetrically disposed on the grid.
    pub fn new_random(
        width: usize,
        height: usize,
        units_per_player: usize,
        my_rand: &mut XorShiftRng,
    ) -> Self {
        let mut g = Game::new(width, height, vec![]);
        g.randomly_add(width, height, units_per_player, my_rand);
        g
    }

    ///
    /// Remove the desired amount of units for the desired player.
    pub fn remove_unit_for_player(&mut self, player: usize, units: usize) -> &mut Self {
        if units > self.players[player].len() {
            panic!("cant remove units for this player");
        }
        for _ in 0..units {
            self.players[player].pop();
        }
        self
    }


    /// Randomly populate the game with units
    /// The units generated will be symmetrically disposed on the grid for each players..
    pub fn randomly_add(
        &mut self,
        width: usize,
        height: usize,
        units_per_player: usize,
        my_rand: &mut XorShiftRng,
    ) {
        self.players = vec![vec![], vec![]];
        for _ in 0..units_per_player {
            let mut unit = Unit::new_random(0, width / 2, height, my_rand, 0);
            while !self.is_free(unit.pos) {
                unit = Unit::new_random(0, width / 2, height, my_rand, 0);
            }
            self.players[1].push(unit.reverse(width, height));
            self.players[0].push(unit);
        }

    }
    /// Returns the list of units for the given player.
    pub fn player(&self, player: usize) -> Vec<Unit> {
        self.players[player].clone()
    }
    /// Returns the width of the game's grid.
    pub fn width(&self) -> usize {
        self.width
    }
    /// Returns the height of the game's grid.
    pub fn height(&self) -> usize {
        self.height
    }
    /// Returns a vector of the positions already used in the game.
    pub fn positions(&self) -> Vec<Position> {
        self.players
            .iter()
            .flat_map(|p| p.iter().map(|u| u.pos.clone()))
            .collect()

    }

    /// We consider that we can only move+shoot one unit every turn.
    /// Returns all the possible moves for the given player.
    pub fn moves(&self, player: usize) -> Vec<Action> {
        let mut mvs = vec![];
        let positions = &self.positions();
        for unit in self.players[player].iter() {
            let mut actions = unit.actions(self.width, self.height, &positions);
            mvs.extend(actions);
        }
        mvs
    }
    /// Returns true if the cell at the given position if empty, false otherwise.
    pub fn is_free(&self, pos: Position) -> bool {
        return self.at(pos).is_none();
    }
    /// Returns the unit at the given position.
    pub fn at(&self, pos: Position) -> Option<Unit> {
        for player in 0..self.players.len() {
            for unit in 0..self.players[player].len() {
                if self.players[player][unit].pos == pos {
                    return Some(self.players[player][unit].clone());
                }
            }
        }
        None
    }
    /// Apply the given move to the game.
    /// It will move the unit and shoot where asked.
    pub fn apply_move(&mut self, action: &Action) {
        for player in 0..self.players.len() {
            for unit in 0..self.players[player].len() {
                if self.players[player][unit].pos == action.shoot {
                    self.players[player].remove(unit);
                    break;
                }
            }
        }
        for player in 0..self.players.len() {
            for unit in 0..self.players[player].len() {
                if self.players[player][unit].pos == action.unit {
                    self.players[player][unit].pos = action.mv;
                }
            }
        }
    }
}

impl fmt::Display for Game {
    /// Formats the game to a user readable format.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "".to_string();
        for x in 0..self.width {
            for y in 0..self.height {
                let mut someone = 0;
                for player in 0..self.players.len() {
                    for unit in 0..self.players[player].len() {
                        if self.players[player][unit].pos.x == x &&
                            self.players[player][unit].pos.y == y
                        {
                            someone = player + 1;
                        }
                    }
                }
                s.push_str(&format!("|{}|", someone));
            }
            s.push_str(&format!("\n"));
        }
        write!(f, "{}", s)
    }
}




#[cfg(test)]
mod test {
    use super::*;
    use unit::{Position, UnitType};
    /// Just checks that the right number of moves are generated.
    #[test]
    fn test_nb_moves() {
        let game = Game::new(
            10,
            10,
            vec![
                vec![],
                vec![
                    Unit::new(Position::new(0, 0), UnitType::Gunner, 1),
                    Unit::new(Position::new(9, 9), UnitType::MobileTower, 1),
                ],
            ],
        );
        assert_eq!(game.moves(0).len(), 0);
        assert_eq!(game.moves(1).len(), 20);
    }
}
