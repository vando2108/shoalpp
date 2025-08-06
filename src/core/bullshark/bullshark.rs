use std::{collections::VecDeque, io::Bytes};

use crate::core::{
    dag::{dag::DAG, vertex::VertexPtr},
    types::TParty,
};

pub type Transaction = Bytes<u8>;

pub struct Bullshark {
    f: TParty,
    dag: DAG<Transaction>,
    buffer: VecDeque<VertexPtr<Transaction>>,
    blocks_to_propose: VecDeque,
}
