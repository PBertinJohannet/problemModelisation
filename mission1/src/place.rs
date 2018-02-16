use shippment::Container;


#[derive(Hash, Clone, PartialEq, Eq, Debug)]
/// A emplacement on the boat.
pub struct Place {
    inner : Vec<Container>,
}
impl Place {
    /// Creates a new empty emplacement.
    pub fn new() -> Self {
        Place{
            inner : vec![],
        }
    }
    /// Checks if the emplacement can accept the given container.
    pub fn can_accept(&self, cont : &Container) -> bool{
        match cont {
            // can not put two munitions at the same place.
            &Container::Munition => !self.inner.iter().any(|c|c == &Container::Munition),
            // can not put artillery on the top of anything.
            &Container::Artillery => self.inner.is_empty(),
            // else we can put anything.
            _ => self.inner.len()<::MAX_PLACE
        }
    }
    /// Push the given container to the emplacement.
    pub fn push(&mut self, cont : Container) {
        self.inner.push(cont);
    }
    /// Get the weight of the place.
    /// we consider that every container weights 1
    pub fn weight(&self) -> i32{
        self.inner.len() as i32
    }
    /// Checks if the place has the given container
    pub fn has(&self, cont : Container) -> bool{
        self.inner.contains(&cont)
    }
}