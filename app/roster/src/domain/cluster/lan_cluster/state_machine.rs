use openraft::{LogId, StoredMembership};
use serde::{Deserialize, Serialize};

use super::{LANNode, NodeRaftID};

/// This state represents a copy of the data between each node. We have to be
/// careful with what is stored here as it'll be shared with every Node of the
/// LocalCluster.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct StateMachine {
    pub last_applied_log: Option<LogId<NodeRaftID>>,
    pub last_membership: StoredMembership<NodeRaftID, LANNode>,
}
