use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemStruct};

/// Implements [`From`] for the given old state, to permit a state transition.
/// 
/// Requires the `Fsm` class as defined below:
/// 
/// ```no_run
/// pub struct Fsm<S> {
///     errors: isize,
///     state: S,
/// }
/// ```
/// 
/// ## Examples
/// ```
/// use crate::retract_fsm_transition::transition_from;
/// 
/// # #[derive(Debug, Eq, PartialEq)]
/// # pub struct Fsm<S> {
/// #     errors: isize,
/// #     pub state: S,
/// # }
/// # 
/// # #[derive(Debug, Eq, PartialEq)]
/// pub struct Standby;
/// 
/// # #[derive(Debug, Eq, PartialEq)] 
/// #[transition_from(Standby)]
/// pub struct Active;
/// 
/// fn main() {
///     let fsm = Fsm {
///         errors: 0,
///         state: Standby 
///     };
///     let fsm2: Fsm<Active> = fsm.into();
///     assert_eq!(fsm2.state, Active);
/// }
/// ```
/// 
/// ```compile_fail
/// # use crate::retract_fsm_transition::transition_from;
/// #
/// # #[derive(Debug, Eq, PartialEq)]
/// # pub struct Fsm<S> {
/// #     errors: isize,
/// #     pub state: S,
/// # }
/// # 
/// # #[derive(Debug, Eq, PartialEq)]
/// pub struct Standby;
///
/// # #[derive(Debug, Eq, PartialEq)] 
/// #[transition_from(Standby)]
/// pub struct Active;
/// 
/// fn main() {
///     let fsm = Fsm {
///         errors: 0,
///         state: Active 
///     };
///     let fsm2: Fsm<Standby> = fsm.into();
///     assert_eq!(fsm2.state, Standby);
/// }
/// ```
#[proc_macro_attribute]
pub fn transition_from(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let old_states = old_state_parser(attr.into());
    let input_ast = parse_macro_input!(input as ItemStruct);
    let new_state = &input_ast.ident;

    let mut from_impls = input_ast.to_token_stream();

    for old in old_states {
        from_impls.extend(quote!(
            impl From<Fsm<#old>> for Fsm<#new_state> {
                fn from(value: Fsm<#old>) -> Self {
                    Self {
                        errors: value.errors,
                        state: #new_state,
                    }
                }
            }
        ))
    }
    
    from_impls.into()
}

fn old_state_parser(attr: TokenStream) -> Vec<Ident> {
    let mut old_states: Vec<Ident> = vec![];

    let mut attr_stream = attr.into_iter();
    let first_state = match attr_stream
        .next()
        .expect("At least one old state must be provided")
    {
        TokenTree::Ident(ident) => ident,
        _ => panic!("Provided attribute is not a valid state"),
    };
    old_states.push(first_state);

    for token in attr_stream {
        match token {
            TokenTree::Ident(ident) => old_states.push(ident),
            TokenTree::Literal(lit) => panic!("Literal {} is not a valid input", lit),
            TokenTree::Punct(_) => (),
            TokenTree::Group(group) => old_states.extend(old_state_parser(group.stream())),
        }
    }

    old_states
}
