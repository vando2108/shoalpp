use std::{cell::RefCell, rc::Rc};

use crate::core::types::TParty;

use super::vertex::VertexPtr;

pub type RoundPtr<T> = Rc<RefCell<Round<T>>>;
pub struct Round<T> {
    vertices: Vec<Option<VertexPtr<T>>>,
}

impl<T> Round<T>
where
    T: Default,
{
    pub fn new(number_parties: TParty) -> Self {
        Round {
            vertices: (0..number_parties).map(|_| None).collect(),
        }
    }
}
