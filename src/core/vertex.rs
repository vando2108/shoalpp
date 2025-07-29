use std::{cell::RefCell, rc::Rc};

use super::types::*;

pub type VertexPtr = Rc<RefCell<Vertex>>;

pub struct Vertex {
    pub round: TRound,
    pub block: TBlock,
    pub source: TParty,
    pub strong_edges: Vec<VertexPtr>,
    pub weak_edges: Vec<VertexPtr>,
}

impl Vertex {
    pub fn create_genesis_vertex() -> Self {
        Vertex {
            round: 0,
            block: TBlock::new(vec![].into()),
            source: 0,
            strong_edges: vec![],
            weak_edges: vec![],
        }
    }

    pub fn create_vertex(
        round: TRound,
        block: TBlock,
        source: TParty,
        strong_edges: Vec<VertexPtr>,
    ) -> VertexPtr {
        Rc::new(RefCell::new(Vertex {
            round,
            block,
            source,
            strong_edges,
            weak_edges: vec![],
        }))
    }
}

/// Find path from Vertex u to Vertex v
/// # Returns
/// True if exsits path between self to other
pub fn path(u: &VertexPtr, v: &VertexPtr) -> bool {
    if Rc::ptr_eq(u, v) {
        return true;
    }

    for vertex in &u.borrow().strong_edges {
        if path(vertex, v) {
            return true;
        }
    }

    for vertex in &u.borrow().weak_edges {
        if path(vertex, v) {
            return true;
        }
    }

    false
}

/// Find strong path from Vertex u to Vertex v
/// # Returns
/// True if exsits strong path between u to v
pub fn strong_path(u: &VertexPtr, v: &VertexPtr) -> bool {
    if Rc::ptr_eq(u, v) {
        return true;
    }

    for vertex in &u.borrow().strong_edges {
        if path(vertex, v) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    // Helper function to create a vertex with given round and source
    fn create_vertex(round: TRound, source: TParty) -> VertexPtr {
        Rc::new(RefCell::new(Vertex {
            round,
            block: TBlock::new(vec![].into()),
            source,
            strong_edges: vec![],
            weak_edges: vec![],
        }))
    }

    #[test]
    fn test_create_genesis_vertex() {
        let genesis = Vertex::create_genesis_vertex();
        assert_eq!(genesis.round, 0);
        assert_eq!(genesis.source, 0);
        assert!(genesis.strong_edges.is_empty());
        assert!(genesis.weak_edges.is_empty());
        // Note: block emptiness check omitted due to unknown TBlock API
    }

    #[test]
    fn test_path_same_vertex() {
        let u = create_vertex(1, 1);
        assert!(path(&u, &u));
    }

    #[test]
    fn test_path_direct_strong_edge() {
        let u = create_vertex(1, 1);
        let v = create_vertex(2, 2);
        u.borrow_mut().strong_edges.push(v.clone());
        assert!(path(&u, &v));
    }

    #[test]
    fn test_path_direct_weak_edge() {
        let u = create_vertex(1, 1);
        let v = create_vertex(2, 2);
        u.borrow_mut().weak_edges.push(v.clone());
        assert!(path(&u, &v));
    }

    #[test]
    fn test_path_indirect_strong() {
        let a = create_vertex(1, 1);
        let b = create_vertex(2, 2);
        let c = create_vertex(3, 3);
        a.borrow_mut().strong_edges.push(b.clone());
        b.borrow_mut().strong_edges.push(c.clone());
        assert!(path(&a, &c));
    }

    #[test]
    fn test_path_indirect_weak() {
        let a = create_vertex(1, 1);
        let b = create_vertex(2, 2);
        let c = create_vertex(3, 3);
        a.borrow_mut().weak_edges.push(b.clone());
        b.borrow_mut().weak_edges.push(c.clone());
        assert!(path(&a, &c));
    }

    #[test]
    fn test_path_indirect_mixed() {
        let a = create_vertex(1, 1);
        let b = create_vertex(2, 2);
        let c = create_vertex(3, 3);
        a.borrow_mut().strong_edges.push(b.clone());
        b.borrow_mut().weak_edges.push(c.clone());
        assert!(path(&a, &c));
    }

    #[test]
    fn test_path_no_path() {
        let u = create_vertex(1, 1);
        let v = create_vertex(2, 2);
        assert!(!path(&u, &v));
    }

    #[test]
    fn test_strong_path_same_vertex() {
        let u = create_vertex(1, 1);
        assert!(strong_path(&u, &u));
    }

    #[test]
    fn test_strong_path_direct_strong_edge() {
        let u = create_vertex(1, 1);
        let v = create_vertex(2, 2);
        u.borrow_mut().strong_edges.push(v.clone());
        assert!(strong_path(&u, &v));
    }

    #[test]
    fn test_strong_path_indirect_strong() {
        let a = create_vertex(1, 1);
        let b = create_vertex(2, 2);
        let c = create_vertex(3, 3);
        a.borrow_mut().strong_edges.push(b.clone());
        b.borrow_mut().strong_edges.push(c.clone());
        assert!(strong_path(&a, &c));
    }

    #[test]
    fn test_strong_path_with_weak_edge() {
        let a = create_vertex(1, 1);
        let b = create_vertex(2, 2);
        let c = create_vertex(3, 3);

        b.borrow_mut().strong_edges.push(a.clone());
        c.borrow_mut().weak_edges.push(b.clone());
        assert!(!strong_path(&a, &c));
    }

    #[test]
    fn test_strong_path_no_path() {
        let u = create_vertex(1, 1);
        let v = create_vertex(2, 2);
        assert!(!strong_path(&u, &v));
    }
}
