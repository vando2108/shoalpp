use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::core::{
    dag::{
        dag::DAG,
        vertex::{Vertex, VertexPtr},
    },
    types::{TParty, TRound},
};

pub type Tx = String;
pub type TxPtr = Rc<RefCell<Tx>>;

pub struct Bullshark {
    party_id: TParty,
    f: TParty,
    wait: bool,
    dag: DAG<TxPtr>,
    buffer: VecDeque<VertexPtr<TxPtr>>,
    blocks_to_propose: VecDeque<TxPtr>,
}

impl Bullshark {
    pub fn new(party_id: TParty, number_parties: TParty, f: TParty) -> Self {
        Bullshark {
            party_id,
            f,
            wait: false,
            dag: DAG::new(number_parties),
            buffer: VecDeque::new(),
            blocks_to_propose: VecDeque::new(),
        }
    }

    pub fn create_new_vertex(&mut self, round: TRound) -> Option<VertexPtr<TxPtr>> {
        let data = self.blocks_to_propose.pop_front();
        if data.is_some() {
            return Some(VertexPtr::new(RefCell::new(Vertex::new(
                round,
                data.unwrap(),
                self.party_id,
            ))));
        }

        None
    }
}
