/// Generates lists of ais with different heuristics to compare them.
use evaluator::{Evaluator, AliveUnitsEvaluator, InfluenceEvaluator, CombinedEvaluator};
use naive::NaiveAI;
use ai::AI;

pub struct EvaluableAI {
    name: String,
    ai: Box<AI>,
}
impl EvaluableAI {
    pub fn new(ai: Box<AI>, name: String) -> Self {
        EvaluableAI {
            name: name.to_string(),
            ai: ai,
        }
    }
    pub fn ai_mut(&mut self) -> &mut AI {
        &mut *self.ai
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}

/// Get all the possibles evaluations functions.
pub fn get_all_evals() -> Vec<(Box<(Evaluator)>, String)> {
    vec![
        (
            influence_eval_double(),
            "double sided influence".to_string()
        ),
        (
            influence_eval_simple(),
            "simple sided influence".to_string()
        ),
        (unit_eval_same(), "unit counting without coef".to_string()),
        (unit_eval_coef(), "unit counting with coef".to_string()),
        (
            unit_eval_double(),
            "double unit counting with coef".to_string()
        ),
        (all_combined(), "all combined".to_string()),
    ]
}


pub fn get_all_naives() -> Vec<EvaluableAI> {
    get_all_evals()
        .into_iter()
        .map(|(eval, s)| {
            EvaluableAI::new(Box::new(NaiveAI::new(eval)), s)
        })
        .collect()
}


/// evaluate the range covered by our units
pub fn influence_eval_simple() -> Box<Evaluator> {
    Box::new(InfluenceEvaluator::new())
}

/// evaluate the range covered by our units minus the range of ennemy units.
pub fn influence_eval_double() -> Box<Evaluator> {
    let mut influence_eval_double = CombinedEvaluator::new();
    influence_eval_double
        .use_ally(Box::new(InfluenceEvaluator::new()))
        .use_ennemy(Box::new(InfluenceEvaluator::new()));
    Box::new(influence_eval_double)
}

/// Counts how many units are still alive but count tower as 5 and gunner as 3
pub fn unit_eval_coef() -> Box<Evaluator> {
    Box::new(AliveUnitsEvaluator::new(5, 1, 3))
}

/// Counts how many of our units are still alive.
pub fn unit_eval_same() -> Box<Evaluator> {
    Box::new(AliveUnitsEvaluator::new(1, 1, 1))
}
/// Counts how many of our units are still alive.
pub fn unit_eval_double() -> Box<Evaluator> {
    let mut unit_eval_double = CombinedEvaluator::new();
    unit_eval_double
        .use_ally(Box::new(AliveUnitsEvaluator::new(1, 1, 1)))
        .use_ennemy(Box::new(AliveUnitsEvaluator::new(1, 1, 1)));
    Box::new(unit_eval_double)
}
/// Counts how many of our units are still alive.
pub fn all_combined() -> Box<Evaluator> {
    let mut unit_eval_double = CombinedEvaluator::new();
    unit_eval_double
        .use_ally(Box::new(AliveUnitsEvaluator::new(10, 10, 10)))
        .use_ennemy(Box::new(AliveUnitsEvaluator::new(10, 10, 10)))
        .use_ally(Box::new(InfluenceEvaluator::new()))
        .use_ennemy(Box::new(InfluenceEvaluator::new()));
    Box::new(unit_eval_double)
}
