use crate::algorithms::value_iteration::ValueTable;
use crate::mdp_traits::{ActionAvailability, ActionEnumerable, Cost, PMass, StatesActions};
use crate::policy::policy_traits::GetActionProbabilityMaybe;
use log::debug;

pub fn policy_evaluation_ssp<'a, M, I, P>(
    mdp: &'a M,
    states: I,
    policy: &P,
    err: f32,
) -> ValueTable<M::State>
where
    M: StatesActions + PMass<f32> + Cost + ActionEnumerable + ActionAvailability,
    I: Iterator<Item = M::State> + Clone,
    P: GetActionProbabilityMaybe<M>,
{
    let mut value_table = ValueTable::<M::State>::new(0.0);
    loop {
        let residual = update_ssp(&mut value_table, mdp, states.clone(), policy);
        debug!("{}", residual);
        debug!("{:?}", value_table);
        if residual < err {
            break;
        }
    }
    value_table
}

fn update_ssp<'a, M, I, P>(vt: &mut ValueTable<M::State>, mdp: &'a M, states: I, policy: &P) -> f32
where
    M: StatesActions + PMass<f32> + Cost + ActionEnumerable + ActionAvailability,
    I: Iterator<Item = M::State>,
    P: GetActionProbabilityMaybe<M>,
{
    let mut max_residual = 0.0;
    for s in states {
        let mut new_value = 0.0;
        for a_id in 0..mdp.num_actions() {
            let a = *mdp.id_to_action(a_id);
            if mdp.action_available(&s, &a) {
                if let Some(p) = policy.get_action_probability_maybe(&s, &a, mdp) {
                    new_value += p * mdp.cost(&s, &a);
                    for (ss, t) in mdp.p_mass(&s, &a) {
                        new_value += p * t * vt.get_value(&ss);
                    }
                }
            }
        }
        let residual = (vt.get_value(&s) - new_value).abs();
        vt.set_value(&s, new_value);

        if residual > max_residual {
            max_residual = residual;
        }
    }

    max_residual
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::value_iteration::value_iteration_ssp;
    use crate::grid_world::*;
    use crate::mdp_traits::*;
    use crate::policy::tabular_policy::TabularPolicy;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_policy_evaluation() {
        let mdp = GridWorldMDP::default();
        let vt = value_iteration_ssp(&mdp);
        let mut tabular_policy = TabularPolicy::from_value_table_ssp(&mdp, &vt);
        let states = mdp.enumerate_states().cloned();
        let evaluated = policy_evaluation_ssp(&mdp, states, &mut tabular_policy, 1e-3);
        for s in mdp.enumerate_states() {
            assert_approx_eq!(vt.get_value(s), evaluated.get_value(s), 1e-3);
        }
    }
}
