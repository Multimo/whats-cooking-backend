use crate::State;

pub type StatefulRequest = tide::Request<State<Repository>>;

pub mod recipes::*;
