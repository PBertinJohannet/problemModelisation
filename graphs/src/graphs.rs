//! contains functions for a star + dijkstra + bfs/dfs.
use std::fmt::Debug;

use std::marker::PhantomData;
use std::collections::{HashSet, VecDeque, HashMap};
use std::hash::Hash;


/// In the bfs/dfs, we don't have any information on cost to a node or to the end.
/// Dijktra is an instance of a star where we dont have informations on
/// the distance from the end.
/// A star is the same with informations on the remaining distance.
pub trait StateNode<U : Clone> {
    /// Returns the transformations to neibourgs states.
    /// The neibourgs can be acceced in one transofmation.
    fn moves(&self) -> Vec<U>;
    /// apply a move on the state.
    fn modify(&mut self, mv: &U);
    /// Returns true if the state is final.
    fn end(&self) -> bool;
    /// Overriding this function will allow you to use dijkstra
    ///
    /// Returns the cost from the current state to the given state.
    fn cost_to(&self, _target: &Self, _mv: &U) -> i32{
        1
    }
    /// Overriding this function will allow you to use a star.
    ///
    /// Must return an estimated distance from the current state to the final state
    /// The distance must satisfy monotony and...
    fn dist_from_end(&self) -> i32{
        1
    }

}


/// Resolves the history of moves to go from *origin* to *start*
/// *history* is a hash map which associate a state to a tuple containing
/// (the preceding state, the move to get there).
pub fn resolve_hist<U: Clone, T: Clone + PartialEq + Eq + Hash>(
    origin: T,
    nouv: T,
    history: HashMap<T, (T, U)>,
) -> Option<Vec<U>> {
    let mut start = &nouv;
    let mut hist = vec![];
    while start != &origin {
        let &(ref pred, ref mv) = history.get(&start).unwrap();
        hist.push(mv.clone());
        start = pred;
    }
    hist.reverse();
    return Some(hist);
}


/// Calling the dijkstra algorithm will just call an a star with no information on the distance to
/// the end
pub type Dijkstra<U : Clone, T : StateNode<U>> = AStar<U, T>;

/// Calling a bfs will just call a dijkstra where the cost of transformations is equal.
pub type BFS<U : Clone, T : StateNode<U>> = AStar<U, T>;



/// Uses dfs to find a way to the final state, there are no guaranties that the path will be the
/// shortest
pub fn dfs<U: Clone, T: StateNode<U> + Hash + Clone + PartialEq + Eq + Debug>(
    state: T,
) -> Option<Vec<U>> {
    let mut history: HashMap<T, (T, U)> = HashMap::new();
    let mut visited = HashSet::new();
    let mut f = VecDeque::new();
    f.push_back(state.clone());
    while let Some(nouv) = f.pop_front() {
        //println!("hist len : {}", history.len());
        if nouv.end() {
            return resolve_hist(state, nouv, history);
        } else {
            visited.insert(nouv.clone());
            for i in nouv.moves() {
                let mut cop = nouv.clone();
                cop.modify(&i);
                if !visited.contains(&cop) {
                    history.insert(cop.clone(), (nouv.clone(), i));
                    f.push_front(cop);
                }
            }
        }
    }
    return None;
}

#[derive(Debug)]
/// THe astar struct containing the elements used during the algorithm.
pub struct AStar<U: Clone, T: StateNode<U> + Hash + Clone + PartialEq + Eq> {
    source: T,
    f_costs: HashMap<T, i32>,
    ongoing: HashSet<T>,
    phantom: PhantomData<U>,
}
impl<U: Clone, T: StateNode<U> + Hash + Clone + PartialEq + Eq + Debug> AStar<U, T> {
    /// Creates a new astar from the given source.
    pub fn new(source: T) -> Self {
        AStar {
            source: source,
            f_costs: HashMap::new(),
            ongoing: HashSet::new(),
            phantom: PhantomData,
        }
    }
    /// Returns the best node, maximizing f_costs
    pub fn best_node(&self) -> T {
        self.ongoing
            .iter()
            .min_by_key(|k| self.f_costs.get(k))
            .unwrap()
            .clone()
    }

    /// Uses astar to find the shortest way to the final state.
    pub fn solve(&mut self) -> Option<Vec<U>> {
        let source = self.source.clone();
        self.ongoing.insert(source.clone());
        let mut lowest_to = HashMap::new();
        lowest_to.insert(source.clone(), 0);
        let mut best_previous = HashMap::new();
        while !self.ongoing.is_empty() {
            let best_node = self.best_node();
            if best_node.end() {
                return resolve_hist(source, best_node, best_previous);
            }
            self.ongoing.remove(&best_node);
            for i in best_node.moves() {
                let mut n = best_node.clone();
                n.modify(&i);
                let new_cost = lowest_to.get(&best_node).unwrap() + best_node.cost_to(&n, &i);
                if !lowest_to.contains_key(&n) || new_cost < *lowest_to.get(&n).unwrap() {
                    lowest_to.insert(n.clone(), new_cost);
                    best_previous.insert(n.clone(), (best_node.clone(), i));
                    self.f_costs.insert(
                        n.clone(),
                        new_cost + best_node.dist_from_end(),
                    );
                    self.ongoing.insert(n.clone());
                }
            }
        }
        None
    }
}


