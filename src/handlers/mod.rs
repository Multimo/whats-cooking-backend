use crate::repo::Repository;
use crate::State;

pub type StatefulRequest = tide::Request<State<Repository>>;

pub mod ingredients;
pub mod recipes;
