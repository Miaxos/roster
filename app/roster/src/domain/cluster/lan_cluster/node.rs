use std::fmt::Display;

/// An implementation of trait [`Node`] that contains minimal node information.
///
/// The most common usage is to store the connecting address of a node.
/// So that an application does not need an additional store to support its
/// [`RaftNetwork`](crate::RaftNetwork) implementation.
///
/// An application is also free not to use this storage and implements its own
/// node-id to address mapping.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct LANNode {
    /// User defined string that represent the endpoint of the target node.
    ///
    /// It is used by [`RaftNetwork`](crate::RaftNetwork) for connecting to
    /// target node.
    pub addr: String,
}

impl LANNode {
    /// Creates as [`BasicNode`].
    pub fn new(addr: impl ToString) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }
}

impl Display for LANNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.addr)
    }
}
