mod node;
mod snapshot;
mod state_machine;
mod store;
use std::io::Cursor;

pub use node::LANNode;
use openraft::storage::Adaptor;
use openraft::MonoioRuntime;
use roster_lan_protocol::{
    RequestEnveloppe, RequestLAN, ResponseEnveloppe, ResponseLAN,
};
use store::Store;

pub type NodeRaftID = u64;

openraft::declare_raft_types!(
    pub TypeConfig: D = RequestLAN, R = ResponseLAN, NodeId = NodeRaftID, Node = LANNode,
    Entry = openraft::Entry<TypeConfig>, SnapshotData = Cursor<Vec<u8>>, AsyncRuntime = MonoioRuntime
);

pub type LogStore = Adaptor<TypeConfig, Store>;
/*
pub type StateMachineStore = Adaptor<TypeConfig, Arc<Store>>;
pub type Raft =
    openraft::Raft<TypeConfig, Network, LogStore, StateMachineStore>;
*/

/// [LANCluster] is the cluster where we'll distribute every hash keys between
/// roster instances
#[derive(Debug, Clone)]
pub struct LANCluster {}
