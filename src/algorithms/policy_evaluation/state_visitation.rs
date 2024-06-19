use crate::algorithms::value_iteration::ValueTable;
use crate::mdp_traits::*;
use crate::policy::policy_traits::GetActionProbabilityMaybe;

pub fn state_visitation_to_value<M, P>(
    mdp: &mut M,
    state_visitation: &ValueTable<M::State>,
    policy: &P,
) -> f32
where
    M: InitialState
        + StatesActions
        + PMass<f32>
        + Rsa
        + StateEnumerable
        + ActionEnumerable
        + ActionAvailability,
    P: GetActionProbabilityMaybe<M>,
{
    let mut result = 0.0;
    for s_id in 0..mdp.num_states() {
        let s = *mdp.id_to_state(s_id);
        for a_id in 0..mdp.num_actions() {
            let a = *mdp.id_to_action(a_id);
            if mdp.action_available(&s, &a) {
                if let Some(p) = policy.get_action_probability_maybe(&s, &a, mdp) {
                    result += p * mdp.rsa(&s, &a) * state_visitation.get_value(&s);
                }
            }
        }
    }
    result
}

pub fn compute_state_visitation<'a, 'b, M, I, P>(
    mdp: &'a M,
    states: I,
    policy: &P,
    gamma: f32,
    err: f32,
) -> ValueTable<M::State>
where
    M: InitialState + StatesActions + PMass<f32> + ActionEnumerable + ActionAvailability,
    I: Iterator<Item = &'b M::State> + Clone,
    P: GetActionProbabilityMaybe<M>,
    M::State: 'b,
{
    let mut cumalative = ValueTable::<M::State>::new(0.0);
    let mut current_visitation = ValueTable::<M::State>::new(0.0);
    cumalative.set_value(&mdp.initial_state(), 1.0);
    current_visitation.set_value(&mdp.initial_state(), 1.0);
    loop {
        let (residual, incoming_flows) = update(
            &mut cumalative,
            mdp,
            states.clone(),
            policy,
            gamma,
            current_visitation,
        );
        current_visitation = incoming_flows;
        if residual < err {
            break;
        }
    }
    cumalative
}

fn update<'a, 'b, M, I, P>(
    vt: &mut ValueTable<M::State>,
    mdp: &'a M,
    states: I,
    policy: &P,
    gamma: f32,
    current_visitation: ValueTable<M::State>,
) -> (f32, ValueTable<M::State>)
where
    M: StatesActions + PMass<f32> + ActionEnumerable + ActionAvailability,
    I: Iterator<Item = &'b M::State> + Clone,
    P: GetActionProbabilityMaybe<M>,
    M::State: 'b,
{
    let mut max_residual = 0.0;
    let mut incoming_flows = ValueTable::<M::State>::new(0.0);
    for s in states.clone() {
        for a_id in 0..mdp.num_actions() {
            let a = *mdp.id_to_action(a_id);
            if mdp.action_available(s, &a) {
                if let Some(p) = policy.get_action_probability_maybe(s, &a, mdp) {
                    for (ss, t) in mdp.p_mass(s, &a) {
                        let flow = current_visitation.get_value(s) * p * t * gamma;
                        if let Some(f) = incoming_flows.value_table.get_mut(&ss) {
                            *f += flow;
                        } else {
                            incoming_flows.set_value(&ss, flow);
                        }
                    }
                }
            }
        }
    }
    for s in states {
        let new_value = incoming_flows.get_value(s);
        //         println!("{:?} {} {}", s, vt.get_value(s), new_value);
        let residual = new_value;
        vt.set_value(s, vt.get_value(s) + new_value);

        if residual > max_residual {
            max_residual = residual;
        }
    }

    (max_residual, incoming_flows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::value_iteration::value_iteration;
    use crate::grid_world::*;
    use crate::policy::tabular_policy::TabularPolicy;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_compute_visitation() {
        let mut mdp = GridWorldMDP::default();
        let vt = value_iteration(&mdp);
        let mut tabular_policy = TabularPolicy::from_value_table(&mdp, &vt);
        let states = mdp.enumerate_states().clone();
        let evaluated = compute_state_visitation(
            &mdp,
            states,
            &mut tabular_policy,
            mdp.get_discount_factor(),
            1e-3,
        );
        for s in mdp.enumerate_states() {
            println!("{:?} {:?}", s, evaluated.get_value(s));
        }
        let value = state_visitation_to_value(&mut mdp, &evaluated, &mut tabular_policy);
        println!("{}", value);
        assert_approx_eq!(value, vt.get_value(&mdp.initial_state()), 1e-3);
    }
}
