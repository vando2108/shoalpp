use thiserror::Error;

use crate::core::types::{TParty, TRound};

#[derive(Error, Debug)]
pub enum DagError {
    #[error("try_add: Invalid vertex")]
    InvalidVertex,

    #[error("try_add: Already exist vertex added by party {0} in round {1}")]
    DuplicateVertexInRound(TParty, TRound),
}
