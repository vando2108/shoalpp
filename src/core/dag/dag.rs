use std::{cell::RefCell, rc::Rc};

use crate::core::types::{TParty, TRound};

use super::{
    errors::DagError,
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
            rounds: vec![RoundPtr::new(RefCell::new(Round::new()))],
        }
    }

    pub fn try_add(&self, v: &VertexPtr<T>) -> Result<bool, DagError> {
        if self.validate_linked_vertices(v) {
            let v_b = v.borrow();
            let mut round_b = self.rounds[v_b.round].borrow_mut();

            round_b.try_add(v)?;
        }

        Err(DagError::InvalidVertex)
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
            for vertex in self.rounds[r].borrow().vertices() {
                if !DAG::path(v, vertex) {
                    v_borrowed.weak_edges.push(vertex.clone());
                }
            }
        }
    }

    fn validate_linked_vertices(&self, v: &VertexPtr<T>) -> bool {
        let v_borrowed = v.borrow();

        // Validate strong_edges
        for vertex in &v_borrowed.strong_edges {
            let vertex_borrowed = vertex.borrow();
            let round = self
                .rounds
                .get(vertex_borrowed.round as usize)
                .and_then(|r| r.borrow().get_vertex(vertex_borrowed.source as usize));

            if let Some(temp_vertex) = round {
                if !Rc::ptr_eq(vertex, &temp_vertex) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Validate weak_edges
        for vertex in &v_borrowed.weak_edges {
            let vertex_borrowed = vertex.borrow();
            let round = self
                .rounds
                .get(vertex_borrowed.round as usize)
                .and_then(|r| r.borrow().get_vertex(vertex_borrowed.source as usize));

            if let Some(temp_vertex) = round {
                if !Rc::ptr_eq(vertex, &temp_vertex) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
