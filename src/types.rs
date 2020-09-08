use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Person {
    pub id: u8,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Workout {
    pub person: Person,
    pub sets: Vec<Set>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Exercise {
    pub name: String,
    pub max: Option<f32>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Set {
    pub exercise: Exercise,
    pub weight: f32,
    pub reps: u8,
}
