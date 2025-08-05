use std::{cell::RefCell, io::Bytes, rc::Rc};

pub type TRound = usize;
pub type TParty = usize;
pub type Transaction = Bytes<u8>;
pub type TBlock = Rc<RefCell<Vec<Transaction>>>;
pub type TWave = usize;
