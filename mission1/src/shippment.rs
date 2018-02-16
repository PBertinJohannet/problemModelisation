/// This will use bfs to find, if possible,
/// a way to ship all containers on the different places without putting anything
/// on an artillery or two munitions together.
use graphs::{StateNode};
use rand::{XorShiftRng, Rng};
use place::Place;
use heuristic::heur;

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
/// The container : A container can contain Munition, Artillery, Light fight equipement and rations
pub enum Container {
    /// There can only be one munition per block.
    Munition,
    /// The artillery must always be at the bottom.
    Artillery,
    /// The light fight equipment.
    LightFight,
    /// The rations.
    Rations,
}
/// Creates a new random container.
pub fn new_container(rand : &mut XorShiftRng) -> Container {
    rand.choose(&[Container::Munition, Container::Artillery, Container::LightFight, Container::Rations]).unwrap().clone()
}

#[derive(Clone, Debug)]
/// A possible movement made by the crane.
pub enum Movement{
    /// The crane can move a container from a stack to another stack.
    StackToStack(usize, usize),
    /// The crane can move a container from a stack to a place in the boat.
    StackToTarget(usize, usize),
}

/// The trait of the stack,
pub trait StackTrait {
    /// The stacks can be created randomly.
    fn new_random(rand : &mut XorShiftRng, size : usize)-> Self;
}
/// A stack is just a vector of containers.
pub type Stack  = Vec<Container>;

impl StackTrait for Stack {
    /// Creates a new random stacks of the desired size with the given random
    /// number generator.
    fn new_random(mut rand : &mut XorShiftRng, size : usize) -> Self {
        (0..size).map(|_|new_container(&mut rand)).collect()
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
/// The shippment, contains the stacks of containers and the places in the boat.
pub struct Shippment {
    places: Vec<Place>,
    stacks: Vec<Vec<Container>>,
}

impl Shippment {
    /// The configurations given in the course as an exercice.
    pub fn new_course() -> Self {
        Shippment::new(
            vec![
                vec![
                    Container::Rations,
                    Container::Rations,
                    Container::Rations,
                    Container::LightFight,
                    Container::LightFight,
                    Container::Munition,
                    Container::Artillery,
                    Container::Artillery,
                ],
                vec![
                    Container::Rations,
                    Container::Rations,
                    Container::Rations,
                    Container::LightFight,
                    Container::LightFight,
                    Container::Munition,
                    Container::Artillery,
                    Container::Artillery,
                ],
                vec![
                    Container::Rations,
                    Container::Rations,
                    Container::Rations,
                    Container::LightFight,
                    Container::LightFight,
                    Container::Munition,
                    Container::Artillery,
                    Container::Artillery,
                ],
                vec![
                    Container::Rations,
                    Container::Rations,
                    Container::LightFight,
                    Container::LightFight,
                    Container::Munition,
                    Container::Munition,
                    Container::Artillery,
                    Container::Artillery,
                ],
            ],
        )

    }
    /// Creates a new random shippment with
    /// nb_places : the number of emplacment on the boat.
    /// nb_stack : the number of stacks.
    /// stacks_size : the size of the stacks.
    pub fn new_random(nb_places : usize, nb_stack : usize, stacks_size : usize, mut rand : &mut XorShiftRng) -> Self {
        Shippment {
            places: (0..nb_places).map(|_| Place::new()).collect(),
            stacks: (0..nb_stack).map(|_| Stack::new_random(&mut rand, stacks_size)).collect(),
        }
    }
    /// Creates a new shippment with the given stats and the default number of emplacement in the boat
    pub fn new( stacks: Vec<Vec<Container>>) -> Self {
        Shippment {
            places: (0..::DEF_NUM_PLACE).map(|_| Place::new()).collect(),
            stacks: stacks,
        }
    }
    /// Returns the weight of the heavier emplacement.
    pub fn max_weight(&self) -> i32 {
        self.places.iter().map(|p|p.weight()).max().unwrap()
    }
    /// Returns a copy of the boat's emplacement.
    pub fn places(&self) -> Vec<Place> {
        self.places.clone()
    }
    /// Returns a copy of the stacks.
    pub fn stacks(&self) -> Vec<Vec<Container>> {
        self.stacks.clone()
    }
}

impl StateNode<Movement> for Shippment {
    /// Checks if stack peeks.
    fn moves(&self) -> Vec<Movement> {
        let mut res = vec![];
        for (id_stack, stack) in self.stacks.iter().enumerate() {
            for (id_target, target) in self.places.iter().enumerate() {
                if !stack.is_empty() && target.can_accept(stack.last().unwrap()) {
                    res.push(Movement::StackToTarget(id_stack, id_target));
                }
            }
            for (id_second_stack, second_stack) in self.stacks.iter().enumerate(){
                if !stack.is_empty() && second_stack != stack && second_stack.len()<12 {
                    res.push(Movement::StackToStack(id_stack, id_second_stack));
                }
            }
        }
        res
    }
    /// apply the move.
    fn modify(&mut self, mv: &Movement) {
        match mv {
            &Movement::StackToStack(stack_1, stack_2) => {
                let container_to_move = self.stacks[stack_1].pop().unwrap();
                self.stacks[stack_2].push(container_to_move);
            },
            &Movement::StackToTarget(stack, target) => {
                let container_to_move = self.stacks[stack].pop().unwrap();
                self.places[target].push(container_to_move);
            }
        }
    }
    /// Checks if we are at the end.
    fn end(&self) -> bool {
        self.stacks.iter().all(|s| s.is_empty())
    }

    /// sums the size of the stacks
    fn dist_from_end(&self)-> i32 {
        heur(self)
        //self.stacks.iter().map(|s| s.len() as i32).sum::<i32>()
    }
}

/// Pretty prints the solution
pub fn print_sol(sol : Vec<Movement>) {
    for mv in sol {
        match mv {
            Movement::StackToTarget(s, t) => println!("bring container on top of stack \
            {} to emplacement {}", s, t),
            Movement::StackToStack(s1, s2) => println!("bring container on top of stack \
            {} to top of stack {}", s1, s2),
        }
    }
}
