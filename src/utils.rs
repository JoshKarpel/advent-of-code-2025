use std::error::Error;

pub type SolverResult = Result<(), Box<dyn Error>>;
