//! The evaluator Trait plus some implementations.
//! The evaluators contain one function to give a score to a game's state.
use unit::UnitType;
use game::Game;
/// The evaluator trait.
pub trait Evaluator {
    /// Given a game and a player, returns a score.
    fn eval(&self, game: &Game, player: usize) -> i32;
}

/// The evaluator counting units alive in my team.
pub struct AliveUnitsEvaluator {
    /// The value of having/killing a tower.
    tower: usize,
    /// The value of having/killing an infantry.
    infantry: usize,
    /// The value of a gunner.
    gunner: usize,
}
impl AliveUnitsEvaluator {
    /// Creates a new alive units evaluator.
    /// Takes the estimated values of the different units.
    pub fn new(tower: usize, infantry: usize, gunner: usize) -> Self {
        AliveUnitsEvaluator {
            tower: tower,
            infantry: infantry,
            gunner: gunner,
        }
    }
}
impl Evaluator for AliveUnitsEvaluator {
    /// Counts the summed value of all units.
    fn eval(&self, game: &Game, player: usize) -> i32 {
        game.player(player)
            .iter()
            .map(|unit| match unit.unit_type {
                UnitType::Infantryman => self.infantry,
                UnitType::MobileTower => self.tower,
                UnitType::Gunner => self.gunner,
            } as i32)
            .sum::<i32>()
    }
}

/// Counts the number of cells in wich The team can shoot.
pub struct InfluenceEvaluator {}
impl InfluenceEvaluator {
    /// Creates a new influence evaluator.
    pub fn new() -> Self {
        InfluenceEvaluator {}
    }
}
impl Evaluator for InfluenceEvaluator {
    /// Calculate the number of cells in wich our team can shoot.
    fn eval(&self, game: &Game, player: usize) -> i32 {
        let mut inf = 0;
        let mut targets = vec![];
        for action in game.moves(player) {
            if !targets.contains(&action.shoot) {
                inf += 1;
                targets.push(action.shoot.clone());
            }
        }
        inf
    }
}

/// The combined evaluator.
/// Allows to combine evaluators to make more complex evaluation function.
/// Also allows to use negative evaluators on the ennemys.
pub struct CombinedEvaluator {
    ally_combined: Vec<Box<Evaluator>>,
    ennemy_combined: Vec<Box<Evaluator>>,
}
impl CombinedEvaluator {
    /// Creates a new empty combined evaluator.
    pub fn new() -> Self {
        CombinedEvaluator {
            ally_combined: vec![],
            ennemy_combined: vec![],
        }
    }
    /// Use an evaluator for the ally.
    pub fn use_ally(&mut self, eval: Box<Evaluator>) -> &mut Self {
        self.ally_combined.push(eval);
        self
    }
    /// Use an evaluator for the ennemy.
    pub fn use_ennemy(&mut self, eval: Box<Evaluator>) -> &mut Self {
        self.ennemy_combined.push(eval);
        self
    }
}
impl Evaluator for CombinedEvaluator {
    /// Use all the given evaluators to calculate the global score of the game state.
    fn eval(&self, game: &Game, player: usize) -> i32 {
        self.ally_combined
            .iter()
            .map(|e| e.eval(game, player))
            .sum::<i32>() -
            self.ennemy_combined
                .iter()
                .map(|e| e.eval(game, 1 - player))
                .sum::<i32>()
    }
}
