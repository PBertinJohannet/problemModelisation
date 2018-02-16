//! The monte carlo tree search algorithm
//!
use game::Game;
use unit::Action;
use ai::{AI, Arena};
use randomai::RandomAI;
use evaluator::Evaluator;

/// A node of the tree.
pub struct Node {
    /// the move used to get to this node.
    move_to: Option<Action>,
    /// The already explored sons of this node.
    sons: Vec<Node>,
    /// The number of wins for this node (for the node)
    wins: usize,
    /// The number of loses for this node.
    loses: usize,
    /// The state of the game for this node.
    game: Game,
    /// The next player who will make a move.
    player: usize,
    /// The possible moves at this node that were not explored.
    possible_moves: Vec<Action>,
}
impl Node {
    /// Creates a new node with the given game state and the given player
    /// Also takes the move used to get to this node.
    pub fn new(game: Game, player: usize, mv: Option<Action>) -> Self {
        let possible_moves = game.moves(player);
        Node {
            move_to: mv,
            sons: vec![],
            wins: 0,
            loses: 0,
            game: game,
            player: player,
            possible_moves: possible_moves,
        }
    }
    /// Choose a node to explore among the sons using the ucb1 algorithm.
    /// Will give a confidence interval of the score of each node.
    ///
    /// Then will choose the node with the highest high bound.
    pub fn ucb1_choose(&mut self, ai: &mut MonteCarloAI, played_games: usize) -> (usize, usize) {
        let games = self.games();
        if self.sons.is_empty() {
            return (1, 0);
        }
        let son = self.sons
            .iter_mut()
            .min_by_key(|son| {
                (100_000_000.0 *
                     (son.weighted_score() as f64 +
                          ((2.0 * games).log(2.7) / (played_games as f64)).sqrt())) as
                    i32

            })
            .unwrap();
        son.selection(ai, played_games)
    }
    /// Returns the number of games played in this node or its descendants.
    pub fn games(&self) -> f64 {
        (self.loses + self.wins) as f64
    }
    /// Finds the best nodes in all the children using a simple evaluation.
    pub fn find_best(&self, ai: &mut MonteCarloAI) -> Action {
        let moves = &self.possible_moves;
        moves
            .iter()
            .map(|mv| {
                let mut g = self.game.clone();
                g.apply_move(mv);
                (mv, ai.eval(&g, self.player))
            })
            .max_by_key(|x| x.1)
            .unwrap()
            .0
            .clone()
    }
    /// Expands the tree.
    /// This will explore a new node, run a random game on it and returns the result in the form
    /// (lose, win).
    pub fn expansion(&mut self, ai: &mut MonteCarloAI, _played_games: usize) -> (usize, usize) {
        let best = self.find_best(ai);
        let index = self.possible_moves.iter().position(|a| a == &best).unwrap();
        let mv = self.possible_moves[index].clone();
        self.possible_moves.remove(index);
        let mut new_game = self.game.clone();
        new_game.apply_move(&mv);
        let mut son = Node::new(new_game, 1 - self.player, Some(mv));
        // simulation.
        let res = son.random_play();
        self.sons.push(son);
        res
    }
    /// If we explored all the possible children nodes, it will select one using the ucb1 algorithm
    /// else it will explore nodes.
    pub fn selection(&mut self, ai: &mut MonteCarloAI, played_games: usize) -> (usize, usize) {
        let (loses, wins) = match self.possible_moves.is_empty() {
            true => self.ucb1_choose(ai, played_games),
            false => self.expansion(ai, played_games),
        };
        self.loses += wins;
        self.wins += loses;
        (wins, loses)
    }
    /// Randomly plays a game and returns the result in the form (lose, win)
    pub fn random_play(&mut self) -> (usize, usize) {
        let won = Arena::resolve(
            &mut self.game.clone(),
            &mut [&mut RandomAI::new(), &mut RandomAI::new()],
            50,
        );
        if won == self.player {
            self.wins += 1;
            (0, 1)
        } else {
            self.loses += 1;
            (1, 0)
        }
    }
    /// Returns the winnig rate
    /// TODO: make this a float pls (wtf ?)
    pub fn weighted_score(&self) -> usize {
        self.wins / (self.wins + self.loses)
    }
    /// Returns the best node by average wins.
    pub fn get_best(&self) -> Action {
        self.sons
            .iter()
            .map(|son| (son.weighted_score(), son.move_to.clone().unwrap()))
            .min_by_key(|x| x.0)
            .unwrap()
            .1
    }
}

/// The monte carlo tree search ai.
pub struct MonteCarloAI {
    /// The evaluation used.
    eval: Box<Evaluator>,
}
impl MonteCarloAI {
    /// reates a new ai.
    pub fn new(eval: Box<Evaluator>) -> Self {
        MonteCarloAI {
            eval: eval,
        }
    }
    pub fn eval(&self, game: &Game, player: usize) -> i32 {
        self.eval.eval(game, player)
    }
}
impl AI for MonteCarloAI {
    /// Explore the tree and returns the result of the best action.
    fn play(&mut self, game: &Game, player: usize) -> Action {
        //println!("mt play");
        let mut node = Node::new(game.clone(), player, None);
        for i in 0..1_000 {
            node.selection(self, i);
        }
        node.get_best()
    }
}
