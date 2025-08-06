use std::{cell::RefCell, rc::Rc};

use crate::core::types::{TParty, TRound};

use super::{
    round::{Round, RoundPtr},
    vertex::VertexPtr,
};

pub struct DAG<T> {
    number_parties: TParty,
    current_round: TRound,
    rounds: Vec<RoundPtr<T>>,
}

impl<T> DAG<T>
where
    T: Default,
{
    pub fn new(number_parties: TParty) -> Self {
        Self {
            number_parties,
            current_round: 1,
            rounds: vec![RoundPtr::new(RefCell::new(Round::new(number_parties)))],
        }
    }

    pub fn path(u: &VertexPtr<T>, v: &VertexPtr<T>) -> bool {
        if Rc::ptr_eq(u, v) {
            return true;
        }

        let u_brrowed = u.borrow();
        u_brrowed
            .strong_edges
            .iter()
            .chain(u_brrowed.weak_edges.iter())
            .any(|vertex| DAG::path(vertex, v))
    }

    pub fn strong_path(u: &VertexPtr<T>, v: &VertexPtr<T>) -> bool {
        if Rc::ptr_eq(u, v) {
            return true;
        }

        let u_brrowed = u.borrow();
        u_brrowed
            .strong_edges
            .iter()
            .any(|vertex| DAG::strong_path(vertex, v))
    }
}
