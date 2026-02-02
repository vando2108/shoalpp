use std::{cell::RefCell, rc::Rc};

use crate::core::types::{TParty, TRound};

pub type VertexPtr<T> = Rc<RefCell<Vertex<T>>>;
pub struct Vertex<T> {
    pub round: TRound,
    pub data: Rc<RefCell<T>>,
    pub source: TParty,
    pub strong_edges: Vec<VertexPtr<T>>,
    pub weak_edges: Vec<VertexPtr<T>>,
}

impl<T> Vertex<T>
where
    T: Default,
{
    pub fn new(round: TRound, data: T, source: TParty, strong_edges: Vec<VertexPtr<T>>) -> Self {
        Vertex {
            round,
            data: Rc::new(RefCell::new(data)),
            source,
            strong_edges,
            weak_edges: vec![],
        }
    }

    pub fn new_genesis() -> Self {
        Vertex {
            round: 0,
            data: Rc::new(RefCell::new(T::default())),
            source: 0,
            strong_edges: vec![],
            weak_edges: vec![],
        }
    }
}
