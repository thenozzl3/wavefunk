
use core::fmt;
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct CoOrd {
    pub x: i32,
    pub  y: i32,
}

impl CoOrd {
    pub const UP: CoOrd = CoOrd { y: 1, x: 0 };
    pub const DOWN: CoOrd = CoOrd { y: -1, x: 0 };
    pub const LEFT: CoOrd = CoOrd { y: 0, x: -1 };
    pub const RIGHT: CoOrd = CoOrd { y: 0, x: 1 };
}

impl fmt::Display for CoOrd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x : {} y: {}", self.x, self.y)
    }
}
