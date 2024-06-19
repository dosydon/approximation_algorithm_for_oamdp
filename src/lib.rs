extern crate mdp;

mod check_solved;
pub mod rtdp;
pub mod rtdp_ensure_convergence_wrapper;
pub mod rtdp_softmax_policy;
// pub mod rtdp_trait;
pub mod lrtdp;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
