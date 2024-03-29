// 2048 - Terminal, 12/03/2023
// Direction: Contains the enum of possible directions and invalid for invalid input
// (c) aichingert

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Invalid
}
