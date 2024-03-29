use openraft::{
    add_async_trait, BasicNode, RaftSnapshotBuilder, Snapshot, SnapshotMeta,
    StorageError, StorageIOError,
};

use super::store::Store;
use super::{NodeRaftID, TypeConfig};

#[derive(Debug)]
pub struct StoredSnapshot {
    pub meta: SnapshotMeta<NodeRaftID, BasicNode>,

    /// The data of the state machine at the time of this snapshot.
    pub data: Vec<u8>,
}

#[add_async_trait]
impl RaftSnapshotBuilder<TypeConfig> for Store {
    async fn build_snapshot(
        &mut self,
    ) -> Result<Snapshot<TypeConfig>, StorageError<NodeRaftID>> {
        todo!()
        /*
                let data;
                let last_applied_log;
                let last_membership;

                {
                    // Serialize the data of the state machine.
                    let state_machine = self.state_machine.read().await;
                    data = serde_json::to_vec(&*state_machine)
                        .map_err(|e| StorageIOError::read_state_machine(&e))?;

                    last_applied_log = state_machine.last_applied_log;
                    last_membership = state_machine.last_membership.clone();
                }

                let snapshot_idx = {
                    let mut l = self.snapshot_idx.lock().unwrap();
                    *l += 1;
                    *l
                };

                let snapshot_id = if let Some(last) = last_applied_log {
                    format!("{}-{}-{}", last.leader_id, last.index, snapshot_idx)
                } else {
                    format!("--{}", snapshot_idx)
                };

                let meta = SnapshotMeta {
                    last_log_id: last_applied_log,
                    last_membership,
                    snapshot_id,
                };

                let snapshot = StoredSnapshot {
                    meta: meta.clone(),
                    data: data.clone(),
                };

                {
                    let mut current_snapshot = self.current_snapshot.write().await;
                    *current_snapshot = Some(snapshot);
                }

                Ok(Snapshot {
                    meta,
                    snapshot: Box::new(Cursor::new(data)),
                })
        */
    }
}
