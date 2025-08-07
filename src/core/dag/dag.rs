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

    pub fn try_add_to_dag(self, v: &VertexPtr<T>) -> bool {
        true
    }

    pub fn path(u: &VertexPtr<T>, v: &VertexPtr<T>) -> bool {
        if Rc::ptr_eq(u, v) {
            return true;
        }

        let u_borrowed = u.borrow();
        u_borrowed
            .strong_edges
            .iter()
            .chain(u_borrowed.weak_edges.iter())
            .any(|vertex| DAG::path(vertex, v))
    }

    pub fn strong_path(u: &VertexPtr<T>, v: &VertexPtr<T>) -> bool {
        if Rc::ptr_eq(u, v) {
            return true;
        }

        let u_borrowed = u.borrow();
        u_borrowed
            .strong_edges
            .iter()
            .any(|vertex| DAG::strong_path(vertex, v))
    }

    pub fn set_weak_edges(self, v: &VertexPtr<T>, round: TRound) {
        let mut v_borrowed = v.borrow_mut();
        v_borrowed.weak_edges = vec![];

        for r in (1..round - 2).rev() {
            for vertex in &self.rounds[r].borrow().vertices {
                if vertex.is_some() {
                    let vertex_unwraped = vertex.as_ref().unwrap();
                    if !DAG::path(v, vertex_unwraped) {
                        v_borrowed.weak_edges.push(vertex_unwraped.clone());
                    }
                }
            }
        }
    }
}
