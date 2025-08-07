use std::{cell::RefCell, collections::VecDeque, io::Bytes, rc::Rc};

use crate::core::{
    dag::{dag::DAG, vertex::VertexPtr},
    types::TParty,
};

pub type Tx = Bytes<u8>;
pub type TxPtr = Rc<RefCell<Tx>>;

pub struct Bullshark {
    f: TParty,
    wait: bool,
    dag: DAG<TxPtr>,
    buffer: VecDeque<VertexPtr<TxPtr>>,
}

impl Bullshark {}
