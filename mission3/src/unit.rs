//! Contains structs and methods to represent units and their actions.
use rand::{XorShiftRng, Rng};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
/// The position's struct.
/// contains a position of a cell.
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    /// Creates a new position.
    pub fn new(x: usize, y: usize) -> Self {
        Position { x: x, y: y }
    }
    /// Move the position by the given values.
    /// If the move cause one coordinate to go under zero, it will underflow, giving a great value.
    ///
    /// This makes it easy to check for underflow.
    pub fn move_by(&mut self, (x, y): (i32, i32)) -> &mut Self {
        self.x = (self.x as i32 + x) as usize;
        self.y = (self.y as i32 + y) as usize;
        self
    }
    /// Given a game's dimension and the already used positions, returns true if the position is
    /// available.
    pub fn valid(&self, width: usize, height: usize, positions: &Vec<Position>) -> bool {
        self.x < width && self.y < height && !positions.contains(&self)
    }
}


#[derive(Clone, Hash, PartialEq, Eq, Debug)]
/// The struct representing an action made by an unit.
pub struct Action {
    /// The position of the unit making that action.
    pub unit: Position,
    /// The position at wich the unit will move.
    pub mv: Position,
    /// The position at wich the unit will shoot.
    pub shoot: Position,
}
impl Action {
    /// Creates a new action with the given parameters.
    pub fn new(unit: Position, mv: (i32, i32), shoot: (i32, i32)) -> Self {
        Action {
            unit: unit,
            mv: unit.clone().move_by(mv).clone(),
            shoot: unit.clone().move_by(shoot).clone(),
        }
    }
    pub fn caster_pos(&self) -> Position {
        self.unit
    }
}


#[derive(Hash, Clone, PartialEq, Eq, Debug)]
/// The enum for the different units.
pub enum UnitType {
    /// The infantry (moves a lot but does not shoot far.
    Infantryman,
    /// The gunner (moves slowly but shoots far.
    Gunner,
    /// The mobile tower (shoots far around and moves slowly).
    MobileTower,
}

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
/// The struct representing the units.
pub struct Unit {
    /// The type of the unit.
    pub unit_type: UnitType,
    /// The position of the unit in the game's grid.
    pub pos: Position,
    /// The team of the unit.
    pub team: usize,
}

impl Unit {
    /// Creates a new unit.
    pub fn new(pos: Position, typ: UnitType, team: usize) -> Self {
        Unit {
            unit_type: typ,
            pos: pos,
            team: team,
        }
    }
    /// Reverse an unit :
    /// Takes an unit at a given position with a team and
    /// returns a new unit at a symmetrical position on the map for the other team.
    pub fn reverse(&self, width: usize, heigth: usize) -> Unit {
        Unit::new(
            Position::new(width - self.pos.x - 1, heigth - self.pos.y - 1),
            self.unit_type.clone(),
            1 - self.team,
        )
    }
    /// Creates a new random unit.
    /// The width_start and width_end represents the range of x positions in wich the unit can be.
    /// The unit's y position will be in [0, height]
    /// Also takes the team of the unit and a random generator.
    pub fn new_random(
        width_start: usize,
        width_end: usize,
        height: usize,
        my_rand: &mut XorShiftRng,
        team: usize,
    ) -> Self {
        Unit {
            unit_type: my_rand
                .choose(
                    &[
                        UnitType::Gunner,
                        UnitType::MobileTower,
                        UnitType::Infantryman,
                    ],
                )
                .unwrap()
                .clone(),
            pos: Position::new(
                my_rand.gen_range(width_start, width_end),
                my_rand.gen_range(0, height),
            ),
            team: team,
        }
    }
    /// Returns a char corresponding to the type of the unit.
    pub fn unit_char(&self) -> char {
        match self.unit_type {
            UnitType::Gunner => 'G',
            UnitType::Infantryman => 'I',
            UnitType::MobileTower => 'T',
        }
    }
    /// Returns the possible moves for the given unit.
    /// width and height are the dimensions of the grid and
    /// pos are the position that are already occupied.
    pub fn actions(&self, width: usize, height: usize, positions: &Vec<Position>) -> Vec<Action> {
        let mut actions = vec![];
        for dep in self.moves() {
            if self.pos.clone().move_by(dep).valid(
                width,
                height,
                positions,
            )
            {
                let mut new_act = self.clone();
                new_act.pos.move_by(dep);
                for shoot in new_act.shoots() {
                    if new_act.pos.clone().move_by(shoot).valid(
                        width,
                        height,
                        &vec![],
                    )
                    {
                        actions.push(Action::new(
                            self.pos,
                            dep,
                            (shoot.0 + dep.0, shoot.1 + dep.1),
                        ));
                    }
                }
            }
        }
        actions
    }
    /// Returns the possible deployments for this unit.
    pub fn moves(&self) -> Vec<(i32, i32)> {
        match self.unit_type {
            UnitType::Infantryman => {
                let mut mvs = vec![];
                for i in 1..3 {
                    mvs.push((0, i));
                    mvs.push((0, -i));
                    mvs.push((-i, 0));
                    mvs.push((i, 0));
                }
                mvs
            }
            _ => vec![(0, 1), (0, -1), (-1, 0), (1, 0)],
        }
    }
    /// Returns the possible shooting targets for this unit at its position.
    fn shoots(&self) -> Vec<(i32, i32)> {
        match self.unit_type {
            UnitType::Infantryman => vec![(0, 1), (0, -1), (-1, 0), (1, 0)],
            UnitType::Gunner => {
                let mut mvs = vec![];
                for i in 1..3 {
                    mvs.push((0, i));
                    mvs.push((0, -i));
                    mvs.push((-i, 0));
                    mvs.push((i, 0));
                }
                mvs
            }
            UnitType::MobileTower => {
                let mut mvs = vec![];
                for i in 1..3 {
                    mvs.push((0, i));
                    mvs.push((0, -i));
                    mvs.push((i, 0));
                    mvs.push((-i, 0));
                    mvs.push((i, i));
                    mvs.push((i, -i));
                    mvs.push((-i, i));
                    mvs.push((-i, -i));
                }
                mvs
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    /// In these tests we assume a grid of size 10*10 and just checks that nothing strange happens
    /// when generating moves..
    #[test]
    fn test_infantry() {
        let mut infantry = Unit::new(Position::new(0, 0), UnitType::Infantryman, 0);
        let last_act = infantry.actions(10, 10, vec![]);
        let mvs: Vec<Position> = last_act.iter().map(|x| x.mv).collect();
        // Every move occur twice because it goes with a possible shot.
        assert_eq!(
            mvs,
            vec![
                Position { x: 0, y: 1 },
                Position { x: 0, y: 1 },
                Position { x: 1, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 0, y: 2 },
                Position { x: 0, y: 2 },
                Position { x: 2, y: 0 },
                Position { x: 2, y: 0 },
            ]
        );
    }
    #[test]
    fn test_mobile_tower() {
        let mut tower = Unit::new(Position::new(0, 0), UnitType::MobileTower, 0);
        let last_act = tower.actions(10, 10, vec![]);
        let mvs: Vec<Position> = last_act.iter().map(|x| x.shoot).collect();
        // Every move occur twice because it goes with a possible shot.
        assert_eq!(
            mvs,
            vec![
                Position { x: 0, y: 1 },
                Position { x: 1, y: 0 },
                Position { x: 1, y: 1 },
                Position { x: 0, y: 2 },
                Position { x: 2, y: 0 },
                Position { x: 2, y: 2 },
                Position { x: 0, y: 1 },
                Position { x: 1, y: 0 },
                Position { x: 1, y: 1 },
                Position { x: 0, y: 2 },
                Position { x: 2, y: 0 },
                Position { x: 2, y: 2 },
            ]
        );
    }
    #[test]
    fn test_gunner() {
        let mut infantry = Unit::new(Position::new(0, 0), UnitType::Gunner, 0);
        let last_act = infantry.actions(10, 10, vec![]);
        println!("moves : {:?}", infantry.moves());
        let mvs: Vec<Position> = last_act.iter().map(|x| x.mv).collect();
        // Every move occur twice because it goes with a possible shot.
        assert_eq!(
            mvs,
            vec![
                Position { x: 0, y: 1 },
                Position { x: 0, y: 1 },
                Position { x: 0, y: 1 },
                Position { x: 0, y: 1 },
                Position { x: 1, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 1, y: 0 },
            ]
        );
    }
}
