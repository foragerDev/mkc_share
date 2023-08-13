use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransitionRegion {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}