#[derive(Debug, Clone, Copy)]
pub enum Budget {
    TimeBudget(f32),
    NumIterations(usize),
}

impl Budget {
    pub fn new(time_budget: Option<f32>, num_iterations: Option<usize>) -> Budget {
        match (time_budget, num_iterations) {
            (Some(t), None) => Budget::TimeBudget(t),
            (None, Some(n)) => Budget::NumIterations(n),
            _ => panic!("Must specify either time budget or number of iterations"),
        }
    }
}
