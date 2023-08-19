use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Direction {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}