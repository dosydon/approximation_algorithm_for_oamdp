use core::fmt::Debug;
use core::hash::Hash;

pub trait State = Eq + PartialEq + Debug + Copy + Clone + Hash;
pub trait Action = Eq + PartialEq + Debug + Copy + Clone + Hash;
pub trait StatesActions {
    type State: State;
    type Action: Action;
}

pub trait IsTerminal: StatesActions {
    fn is_terminal(&self, s: &Self::State) -> bool;
}

// pub trait RMax {
//     fn r_max(&self) -> f32;
// }
//
// pub trait RMin {
//     fn r_min(&self) -> f32;
// }

pub trait DiscountFactor {
    fn get_discount_factor(&self) -> f32;
}

pub trait Rsa: StatesActions {
    fn rsa(&self, s: &Self::State, a: &Self::Action) -> f32;
}

pub trait Rsas: StatesActions {
    fn rsas(&self, s: &Self::State, a: &Self::Action, ss: &Self::State) -> f32;
}

pub trait Build<Item: Sized> {
    fn build(self) -> Item;
}

pub trait BuildFrom<Parameter: Sized, Item: Sized> {
    fn build_from(&self, from: Parameter) -> Item;
}

pub trait ActionAvailability: StatesActions {
    fn action_available(&self, _s: &Self::State, _a: &Self::Action) -> bool {
        true
    }
}

pub trait ToVarName {
    fn to_var_name(&self) -> String;
}

pub fn to_var_name<S: ToVarName, A: ToVarName>(s: &S, a: &A) -> String {
    format!("{}_{}", s.to_var_name(), a.to_var_name())
}

pub trait DisplayState<S> {
    fn display(&self, s: &S);
}

pub trait RenderTo: StatesActions {
    fn render_to(&self, s: &Self::State, path: &str);
}
