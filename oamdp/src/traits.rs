use core::fmt::Debug;
use core::hash::Hash;
use mdp::{
    finite_horizon_wrapper::{FiniteHorizonWrapper, FiniteHorizonWrapperState},
    mdp_traits::StatesActions,
};
use ordered_float::*;
use std::slice::Iter;

pub trait BeliefOverGoal<const N: usize> {
    fn get_belief_over_goal(&self) -> [NotNan<f32>; N];
}

impl<S: Eq + PartialEq + Debug + Hash + Copy + BeliefOverGoal<N>, const N: usize> BeliefOverGoal<N>
    for FiniteHorizonWrapperState<S>
{
    fn get_belief_over_goal(&self) -> [NotNan<f32>; N] {
        self.s.get_belief_over_goal()
    }
}

pub trait ProbSassGivenTheta<S, A> {
    fn prob_sass_given_theta(self, id: usize, s: &S, a: &A, ss: &S) -> f32;
}

pub trait DomainAction {
    type DomainAction: Eq + PartialEq + Debug + Copy + Clone + Hash;
}

impl<M: DomainAction + StatesActions> DomainAction for FiniteHorizonWrapper<M> {
    type DomainAction = M::DomainAction;
}

pub trait EnumerateDomainAction: DomainAction {
    fn enumerate_domain_actions(&self) -> Iter<Self::DomainAction>;
}

impl<M: EnumerateDomainAction + StatesActions> EnumerateDomainAction for FiniteHorizonWrapper<M> {
    fn enumerate_domain_actions(&self) -> Iter<Self::DomainAction> {
        self.mdp.enumerate_domain_actions()
    }
}

pub trait Message {
    type Message: Eq + PartialEq + Debug + Copy + Clone + Hash;
}

impl<M: Message + StatesActions> Message for FiniteHorizonWrapper<M> {
    type Message = M::Message;
}

pub trait EnumerateMessage: Message {
    fn enumerate_message(&self) -> Iter<Self::Message>;
}

impl<M: EnumerateMessage + StatesActions> EnumerateMessage for FiniteHorizonWrapper<M> {
    fn enumerate_message(&self) -> Iter<Self::Message> {
        self.mdp.enumerate_message()
    }
}

pub trait Set<A: Eq + PartialEq + Debug + Copy + Clone + Hash> {
    fn set(&mut self, m: A);
}

pub trait CommunicationCost: StatesActions {
    fn communication_cost(&self, s: &Self::State, a: &Self::Action) -> f32;
}

pub trait CommunicationProbability<M> {
    fn communication_probability(&self, id: usize, m: &M) -> f32;
}
