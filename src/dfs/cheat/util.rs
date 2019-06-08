use std::sync::atomic::{AtomicUsize, Ordering};
use std::usize;

/// State of a vertex as known from a `root`.
#[derive(Debug, Clone, Copy)]
pub enum VertState {
    /// No information. Must look at the shared `owner` array to determine.
    Unknown,
    /// Not owned by `root`. Vertex is owned by another tree.
    NotOwned,
    /// Owned by `root`, but not yet visited in the traversal.
    OwnedUnused,
    /// Owned by `root` and visited in the traversal.
    OwnedUsed,
}

impl VertState {
    pub fn is_owned_unused(&self) -> bool {
        match *self {
            VertState::NotOwned => false,
            VertState::OwnedUnused => true,
            VertState::OwnedUsed => false,
            VertState::Unknown => unreachable!()
        }
    }
}

/// What a particilar root knows about the rest of the graph.
///
/// This serves both as a local cache of the values in the shared
/// `owner` array and as a `used` array.
pub struct State<'a> {
    root: usize,
    state: Vec<VertState>,
    owner: &'a [AtomicUsize],
}

impl<'a> State<'a> {
    pub fn new(root: usize, owner: &'a [AtomicUsize]) -> Self {
        let mut state = vec![VertState::Unknown; owner.len()];
        state[root] = VertState::OwnedUsed;

        State {
            root,
            state,
            owner,
        }
    }

    pub fn get(&mut self, v: usize) -> VertState {
        if let VertState::Unknown = self.state[v] {
            if take_ownership(&self.owner[v], self.root) {
                self.state[v] = VertState::OwnedUnused;
            } else {
                self.state[v] = VertState::NotOwned;
            }
        }

        self.state[v]
    }

    pub fn mark_used(&mut self, v: usize) {
        match self.state[v] {
            VertState::OwnedUnused => self.state[v] = VertState::OwnedUsed,
            VertState::OwnedUsed => (),
            _ => panic!("vertex must be owned")
        }
    }
}

pub fn take_ownership(owner: &AtomicUsize, root: usize) -> bool {
    let current = owner.compare_and_swap(usize::MAX, root, Ordering::Relaxed);
    current == usize::MAX
}
