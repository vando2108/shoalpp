use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    rc::Rc,
    thread::park_timeout,
};

use super::{
    types::{TBlock, TParty, TRound, TWave},
    vertex::{Vertex, VertexPtr, path},
};

pub struct Round {
    vertices: Vec<VertexPtr>,
}

pub type RoundPtr = Rc<RefCell<Round>>;

pub struct DAG {
    f: usize,
    number_parties: TParty,
    round: TRound,
    rounds: Vec<RoundPtr>,
    buffer: VecDeque<VertexPtr>,
    wait: bool,
    blocks_to_propose: VecDeque<TBlock>,
    party_id: TParty,
}

impl DAG {
    pub fn new(party_id: TParty, number_parties: TParty, f: usize) -> Self {
        let genesis_vertices: Vec<VertexPtr> = (0..number_parties)
            .map(|_| VertexPtr::new(Vertex::create_genesis_vertex().into()))
            .collect();

        let genesis_round: RoundPtr = Rc::new(RefCell::new(Round {
            vertices: genesis_vertices,
        }));

        Self {
            f,
            party_id,
            number_parties,
            round: 1,
            rounds: vec![genesis_round],
            wait: true,
            buffer: VecDeque::new(),
            blocks_to_propose: VecDeque::new(),
        }
    }

    pub fn deliver(&mut self, v: &VertexPtr, round: TRound, p: TParty) {
        let v_brrow = v.borrow();

        #[allow(clippy::int_plus_one)]
        if v_brrow.source == p
            && v_brrow.round == round
            && v_brrow.strong_edges.len() >= 2 * self.f + 1
        {
            if !self.try_add_to_dag(v) {
                self.buffer.push_back(v.clone());
            } else {
                while !self.buffer.is_empty() {
                    let vertex = self.buffer.pop_front().unwrap();
                    if vertex.borrow().round <= round && !self.try_add_to_dag(&vertex) {
                        self.buffer.push_front(vertex);
                    }
                }
            }

            if round == self.round {
                let wave = (round as f64 / 4_f64).ceil() as TWave;

                if round % 4 == 1 {
                    let mut go_next_round = !self.wait;
                    if !go_next_round {
                        let first_steady_leader =
                            self._get_first_leader_vertex_leader(wave).unwrap();
                        for vertex in &self.rounds[round].borrow().vertices {
                            if vertex.borrow().source == self._get_first_leader_vertex_leader(wave)
                            {
                                flag = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn try_add_to_dag(&mut self, v: &VertexPtr) -> bool {
        if !self._check_all_adj_in_dag(v) {
            return false;
        }

        let brrow_v = v.borrow();
        let mut current_round = self.rounds[self.round].borrow_mut();
        current_round.vertices.insert(brrow_v.source, v.clone());

        #[allow(clippy::int_plus_one)]
        if current_round.vertices.len() >= 2 * self.f + 1 && v.borrow().round > self.round {
            self.round = brrow_v.round;
            // TODO: Start timer
            self.wait = true;
        }

        // TODO: Consider to add self.buffer.pop_front
        self.try_ordering();

        true
    }

    pub fn broadcast_vertex(&mut self, round: TRound) {
        let v = self.create_new_vertex(round);
        self.try_add_to_dag(&v);
        // TODO: r_bcast(v, round)
    }

    pub fn try_ordering(&self) {}

    pub fn create_new_vertex(&mut self, round: TRound) -> VertexPtr {
        // TODO: wait until self.blocks_to_propose.not_empty()
        let block = self.blocks_to_propose.pop_front();

        let vertex = Vertex::create_vertex(
            round,
            block.unwrap(),
            self.party_id,
            self.rounds[round - 1].borrow().vertices.clone(),
        );
        self._set_weak_edges(&vertex, round);

        vertex
    }

    fn _check_all_adj_in_dag(&self, v: &VertexPtr) -> bool {
        for round in &self.rounds {
            for round_vertex in &round.borrow().vertices {
                for vertex in &v.borrow().strong_edges {
                    if !Rc::ptr_eq(vertex, round_vertex) {
                        return false;
                    }
                }

                for vertex in &v.borrow().weak_edges {
                    if !Rc::ptr_eq(vertex, round_vertex) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn _set_weak_edges(&self, v: &VertexPtr, round: TRound) {
        for r in (1..round - 2).rev() {
            for vertex in &self.rounds[r].borrow().vertices {
                if path(v, vertex) {
                    v.borrow_mut().weak_edges.push(vertex.clone());
                }
            }
        }
    }

    fn _get_vertex(&self, party: TParty, round: TRound) -> Result<VertexPtr, String> {
        if round > self.round {
            return Err("The round can't large than the DAG.current_round".to_string());
        }

        for vertex in &self.rounds[round].borrow().vertices {
            if vertex.borrow().source == party {
                return Ok(vertex.clone());
            }
        }

        Err("Can't find".to_string())
    }

    fn _get_fallback_vertex_leader(&self, wave: TWave) -> Result<VertexPtr, String> {
        let party: TParty = (2 * wave) % self.number_parties;
        self._get_vertex(party, 4 * wave - 3)
    }

    fn _get_first_leader_vertex_leader(&self, wave: TWave) -> Result<VertexPtr, String> {
        let party: TParty = (2 * wave + 1) % self.number_parties;
        self._get_vertex(party, 4 * wave - 3)
    }

    fn _get_second_leader_vertex_leader(&self, wave: TWave) -> Result<VertexPtr, String> {
        let party: TParty = (2 * wave + 2) % self.number_parties;
        self._get_vertex(party, 4 * wave - 1)
    }
}
