use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::core::types::TParty;

use super::{errors::DagError, vertex::VertexPtr};

pub type RoundPtr<T> = Rc<RefCell<Round<T>>>;
pub struct Round<T> {
    marker: HashMap<TParty, usize>,
    vertices: Vec<VertexPtr<T>>,
}

impl<T> Round<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Round {
            marker: HashMap::new(),
            vertices: vec![],
        }
    }

    pub fn count(self) -> usize {
        self.vertices.len()
    }

    pub fn vertices(&self) -> &Vec<VertexPtr<T>> {
        &self.vertices
    }

    pub fn get_vertex(&self, source: TParty) -> Option<VertexPtr<T>> {
        if let Some(pos) = self.marker.get(&source) {
            return Some(self.vertices.get(*pos).unwrap().clone());
        }

        None
    }

    pub fn is_existed(&self, source: TParty) -> bool {
        self.marker.contains_key(&source)
    }

    pub fn try_add(&mut self, v: &VertexPtr<T>) -> Result<(), DagError> {
        let v_b = v.borrow();
        if self.is_existed(v_b.source) {
            return Err(DagError::DuplicateVertexInRound(v_b.source, v_b.round));
        }

        self.vertices.push(v.clone());
        self.marker.insert(v_b.source, self.vertices.len());

        Ok(())
    }
}
