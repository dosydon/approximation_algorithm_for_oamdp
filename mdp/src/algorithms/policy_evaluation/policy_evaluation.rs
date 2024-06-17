use crate::algorithms::value_iteration::ValueTable;
use crate::mdp_traits::*;
use crate::policy::policy_traits::PartialPolicy;
use log::debug;

pub fn policy_evaluation<'a, 'b, M, I, P>(
    mdp: &'a M,
    states: I,
    policy: &P,
    err: f32,
) -> ValueTable<M::State>
where
    M: StatesActions + PMass<f32> + Rsa + DiscountFactor + ActionEnumerable + ActionAvailability,
    I: Iterator<Item = &'b M::State> + Clone,
    P: PartialPolicy<M>,
    M::State: 'b,
{
    let mut value_table = ValueTable::<M::State>::new(0.0);
    loop {
        let residual = update(&mut value_table, mdp, states.clone(), policy);
        debug!("{}", residual);
        debug!("{:?}", value_table);
        if residual < err {
            break;
        }
    }
    value_table
}

fn update<'a, 'b, M, I, P>(vt: &mut ValueTable<M::State>, mdp: &'a M, states: I, policy: &P) -> f32
where
    M: StatesActions + PMass<f32> + Rsa + DiscountFactor + ActionEnumerable + ActionAvailability,
    I: Iterator<Item = &'b M::State>,
    P: PartialPolicy<M>,
    M::State: 'b,
{
    let mut max_residual = 0.0;
    for s in states {
        let mut new_value = 0.0;
        for a in mdp.enumerate_actions() {
            if mdp.action_available(s, a) {
                if let Some(p) = policy.get_probability_maybe(s, a, mdp) {
                    new_value += p * mdp.rsa(s, a);
                    for (ss, t) in mdp.p_mass(s, a) {
                        new_value += p * t * mdp.get_discount_factor() * vt.get_value(&ss);
                    }
                }
            }
        }
        let residual = (vt.get_value(s) - new_value).abs();
        vt.set_value(s, new_value);

        if residual > max_residual {
            max_residual = residual;
        }
    }

    max_residual
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::value_iteration::value_iteration;
    use crate::grid_world::*;
    use crate::policy::tabular_policy::TabularPolicy;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_policy_evaluation() {
        let mdp = GridWorldMDP::default();
        let vt = value_iteration(&mdp);
        let tabular_policy = TabularPolicy::from_value_table(&mdp, &vt);
        let evaluated = policy_evaluation(&mdp, mdp.enumerate_states(), &tabular_policy, 1e-3);
        for s in mdp.enumerate_states() {
            assert_approx_eq!(vt.get_value(s), evaluated.get_value(s), 1e-3);
        }
    }
}
