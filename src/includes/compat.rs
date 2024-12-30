use crate::includes::coord::CoOrd;
use core::fmt;
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Compat {
   pub tile1: char,
   pub tile2: char,
   pub direction: CoOrd,
}


impl fmt::Display for Compat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.tile1, self.tile2, self.direction.x, self.direction.y
        )
    }
}
