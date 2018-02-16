use std::fmt::Debug;
use graphs::graphs::StateNode;
use itertools::Itertools;

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
/// A state of the problem.
/// contains boat at the beggining and the end.
/// if there are boats at the beggining, then the support team is at the beggining.
pub struct Harbour {
    begining: Vec<i32>,
    arival: Vec<i32>,
}

impl Harbour {
    /// creates a new instance of the problem with the given boats at the beggining.
    pub fn new(ships: Vec<i32>) -> Self {
        Harbour {
            begining: ships,
            arival: vec![],
        }
    }
}

/// Removes an element from a vector.
pub fn remove_at<U: Eq + Debug>(vec: &mut Vec<U>, elem: &U) {
    let to_rem = vec.iter().position(|s| s == elem).unwrap();
    vec.remove(to_rem);
}

impl StateNode<(i32, i32)> for Harbour {
    /// The moves here are just sending two boats to the end
    /// the fastest boat at the end will return.
    fn moves(&self) -> Vec<(i32, i32)> {
        self.begining.iter().combinations(2).map(|v| (*v[0], *v[1])).collect()
    }
    /// Takes a move and modify the game with it.
    /// Removes the boats from the beginning and put them at the end.
    /// Then if there are still boats at the beggining, brings the fastest ship at the end back.
    fn modify(&mut self, mv: &(i32, i32)) {
        remove_at(&mut self.begining, &mv.0);
        remove_at(&mut self.begining, &mv.1);
        self.arival.push(mv.0);
        self.arival.push(mv.1);
        if !self.end() {
            let fastest = self.arival.iter().min().unwrap().clone();
            remove_at(&mut self.arival, &fastest);
            self.begining.push(fastest);
        }
    }
    /// If the beggining is empty, we found a solution.
    fn end(&self) -> bool {
        self.begining.is_empty()
    }

    /// Calculates the distance bewteen the current state and the target state.
    fn cost_to(&self, _target: &Self, mv: &(i32, i32)) -> i32 {
        let maxs = vec![mv.0, mv.1, self.arival.iter().min().unwrap_or(&100).clone()];
        let min_ret = match self.begining.len()==2 {
            true => maxs.iter().min().unwrap(),
            false => &0,
        };
        [mv.0, mv.1].iter().max().unwrap() + min_ret
    }

    /// Proven to respect monotony.
    fn dist_from_end(&self) -> i32 {
        ((self.begining.iter().sum::<i32>() as i32) - (self.begining.iter().min().unwrap()))/2
    }
}

/// Pretty Prints the solution
pub fn print_sol(solution : Vec<(i32, i32)>) {
    for mv in solution {
        println!("brings the two boats of size : {}, {} to the end", mv.0, mv.1);
        println!("bring back the support team on the fastest possible boat.");
    }
}