/// The module in wich we can find the heuristic used in the a star algorithm
///
use shippment::{Container, Shippment};
/// The currently used heuristic.
pub fn heur(shippment : &Shippment) -> i32{
    let mut score = 0.0;
    // the best way is to count exactly the number of boxes missing.
    for stack in shippment.stacks(){
        for cont in stack.iter(){
            score += match cont {
                &Container::Artillery => 1.0,
                &Container::Munition => 1.0,
                _ => 1.0,

            }
        }
    }

    score as i32
}
