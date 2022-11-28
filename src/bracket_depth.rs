use std::ops::{AddAssign, Add};

#[derive(Default, Copy, Clone, PartialEq)]
pub struct BrackDepths {
    pub curly: i32,
    pub square: i32,
    pub round: i32,
}

impl Add for BrackDepths {
    type Output = BrackDepths;

    fn add(self, other: BrackDepths) -> BrackDepths {
        BrackDepths {
            curly: self.curly + other.curly,
            square: self.square + other.square,
            round: self.round + other.round,
        }
    }
}

impl AddAssign for BrackDepths {
    fn add_assign(&mut self, other: BrackDepths) {
        self.curly += other.curly;
        self.square += other.square;
        self.round += other.round;
    }
}

impl BrackDepths {
    pub fn is_zero(&self) -> bool {
        self.curly == 0 && self.square == 0 && self.round == 0
    }
}