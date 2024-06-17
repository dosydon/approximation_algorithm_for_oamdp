use proc_macro::{self, TokenStream};
use quote::quote;
use syn::parse_macro_input;
use syn::parse_quote;
use syn::DeriveInput;

#[proc_macro_derive(StatesActions)]
pub fn derive_states_actions(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(StatesActions));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics StatesActions for #st_name #ty_generics {
            type State = M::State;
            type Action = M::Action;
        }
    };
    output.into()
}

#[proc_macro_derive(IsTerminal)]
pub fn derive_is_terminal(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(IsTerminal));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics IsTerminal for #st_name #ty_generics {
            fn is_terminal(&self, s: &Self::State) -> bool {
                self.mdp.is_terminal(s)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(StateEnumerable)]
pub fn derive_state_enumerable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(StateEnumerable));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics StateEnumerable for #st_name #ty_generics {
            fn enumerate_states(&self) -> Iter<Self::State> {
                self.mdp.enumerate_states()
            }
            fn num_states(&self) -> usize {
                self.mdp.num_states()
            }
            fn id_to_state(&self, id: usize) -> &Self::State {
                self.mdp.id_to_state(id)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(ActionEnumerable)]
pub fn derive_action_enumerable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(ActionEnumerable));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics ActionEnumerable for #st_name #ty_generics {
            fn enumerate_actions(&self) -> Iter<Self::Action> {
                self.mdp.enumerate_actions()
            }
            fn num_actions(&self) -> usize {
                self.mdp.num_actions()
            }
            fn id_to_action(&self, id: usize) -> &Self::Action {
                self.mdp.id_to_action(id)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(ActionAvailability)]
pub fn derive_action_availability(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(ActionAvailability));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics ActionAvailability for #st_name #ty_generics {
            fn action_available(&self, s: &Self::State, a: &Self::Action) -> bool {
                self.mdp.action_available(s, a)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(InitialState)]
pub fn derive_initial_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(InitialState));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics InitialState for #st_name #ty_generics {
            fn initial_state(&self) -> Self::State {
                self.mdp.initial_state()
            }
        }
    };
    output.into()
}

#[proc_macro_derive(Cost)]
pub fn derive_cost(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(Cost));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics Cost for #st_name #ty_generics {
            fn cost(&self, s: &Self::State, a: &Self::Action) -> f32 {
                self.mdp.cost(s, a)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(DCost)]
pub fn derive_d_cost(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;

    let output = quote! {
        impl<M: DCost> DCost for #st_name<M> {
            fn d_cost(&self, s: &Self::State, a: &Self::Action, ss: &Self::State) -> f32 {
                self.mdp.d_cost(s, a, ss)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(PreferredSuccessor)]
pub fn derive_preferred_successor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;

    let output = quote! {
        impl<M: PreferredSuccessor> PreferredSuccessor for #st_name<M> {
            fn preferred_successor(&self, s: &Self::State, a: &Self::Action) -> Self::State {
                self.mdp.preferred_successor(s, a)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(PMass32)]
pub fn derive_generative_mdp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(PMass<f32>));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics PMass<f32> for #st_name #ty_generics {
            type Distribution = <M as PMass<f32>>::Distribution;
            fn p_mass(&self, s: &Self::State, a: &Self::Action) -> Self::Distribution {
                self.mdp.p_mass(s, a)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(GenerativeMDPMut)]
pub fn derive_generative_mut_mdp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(GenerativeMDPMut));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics GenerativeMDPMut for #st_name #ty_generics {
            fn p_mass_mut(&mut self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f32)> {
                self.mdp.p_mass_mut(s, a)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(GenerativeMDPMut64)]
pub fn derive_generative_mut_64_mdp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(GenerativeMDPMut64));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics GenerativeMDPMut64 for #st_name #ty_generics {
            fn p_mass_mut_64(&mut self, s: &Self::State, a: &Self::Action) -> Vec<(Self::State, f64)> {
                self.mdp.p_mass_mut_64(s, a)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(ExplicitTransition)]
pub fn derive_explicit_transition(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(ExplicitTransition));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics ExplicitTransition for #st_name #ty_generics {
            fn p(&self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
                self.mdp.p(st, a, stt)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(ExplicitTransitionMut)]
pub fn derive_explicit_transition_mut(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(ExplicitTransitionMut));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics ExplicitTransitionMut for #st_name #ty_generics {
            fn p_mut(&mut self, st: &Self::State, a: &Self::Action, stt: &Self::State) -> f32 {
                self.mdp.p_mut(st, a, stt)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(Inner)]
pub fn derive_into_inner(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;

    let output = quote! {
        impl Inner for #st_name {
            type Result = Self;
            fn inner(&self) -> Self::Result {
                *self
            }
        }
    };
    output.into()
}

#[proc_macro_derive(InnerMost)]
pub fn derive_into_inner_most(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;

    let output = quote! {
        impl InnerMost for #st_name {
            type Result = Self;
            fn inner_most(&self) -> Self::Result {
                *self
            }
        }
    };
    output.into()
}

#[proc_macro_derive(Rsa)]
pub fn derive_reward_mdp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(Rsa));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics Rsa for #st_name #ty_generics {
            fn rsa(&self, s: &Self::State, a: &Self::Action) -> f32 {
                self.mdp.rsa(s, a)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(DiscountFactor)]
pub fn derive_discount_factor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(DiscountFactor));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics DiscountFactor for #st_name #ty_generics {
            fn get_discount_factor(&self) -> f32 {
                self.mdp.get_discount_factor()
            }
        }
    };
    output.into()
}

#[proc_macro_derive(GetNextState)]
pub fn derive_get_next_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let mut st_generics = input.generics;
    if let Some(t) = st_generics.type_params_mut().nth(0) {
        t.bounds.push(parse_quote!(GetNextState));
    }
    let (impl_generics, ty_generics, _where_clause) = st_generics.split_for_impl();

    let output = quote! {
        impl #impl_generics GetNextState for #st_name #ty_generics {
            fn get_next_state(&self, s: &Self::State, a: &Self::Action, rng: &mut ThreadRng) -> Self::State {
                self.mdp.get_next_state(s, a, rng)
            }
        }
    };
    output.into()
}
