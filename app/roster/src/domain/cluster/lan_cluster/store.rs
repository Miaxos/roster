use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::io::Cursor;
use std::ops::RangeBounds;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use openraft::{
    add_async_trait, AnyError, Entry, EntryPayload, ErrorSubject, ErrorVerb,
    LogId, LogState, RaftLogReader, RaftStorage, RaftTypeConfig, Snapshot,
    SnapshotMeta, StorageError, StorageIOError, StoredMembership, Vote,
};
use roster_lan_protocol::{RequestLAN, ResponseEnveloppe, ResponseLAN};
use scc::ebr::Guard;
use scc::TreeIndex;

use super::state_machine::StateMachine;
use super::{LANNode, NodeRaftID, TypeConfig};

#[derive(Debug, Default, Clone)]
pub struct Store {
    last_purged_log_id: Rc<Cell<Option<LogId<NodeRaftID>>>>,
    log: Rc<TreeIndex<u64, Entry<TypeConfig>>>,
    /// The current granted vote.
    vote: Rc<Cell<Option<Vote<NodeRaftID>>>>,
    /// The Raft state machine.
    state_machine: Rc<RefCell<StateMachine>>,
}

#[add_async_trait]
impl RaftLogReader<TypeConfig> for Store {
    async fn try_get_log_entries<RB: RangeBounds<u64> + Clone + Debug>(
        &mut self,
        range: RB,
    ) -> Result<Vec<Entry<TypeConfig>>, StorageError<NodeRaftID>> {
        let guard = Guard::new();
        let response = self
            .log
            .range(range, &guard)
            // Useless step that we could work on openraft.
            .map(|(_, val)| val.clone())
            .collect::<Vec<_>>();

        Ok(response)
    }
}
#[add_async_trait]
impl RaftStorage<TypeConfig> for Store {
    type LogReader = Self;
    type SnapshotBuilder = Self;

    async fn get_log_state(
        &mut self,
    ) -> Result<LogState<TypeConfig>, StorageError<NodeRaftID>> {
        let guard = Guard::new();
        // Not sure if this is the most efficient maybe we could clone and store
        // it.
        let last = self.log.iter(&guard).last().map(|(_, ent)| ent.log_id);
        let last_purged = self.last_purged_log_id.get();

        let last = match last {
            None => last_purged,
            Some(x) => Some(x),
        };

        Ok(LogState {
            last_purged_log_id: last_purged,
            last_log_id: last,
        })
    }

    async fn save_vote(
        &mut self,
        vote: &Vote<NodeRaftID>,
    ) -> Result<(), StorageError<NodeRaftID>> {
        // Only stored in-memory, to ensure correctness we must save it on the
        // disk.
        //
        // TODO: Change it to `io`
        self.vote.set(Some(*vote));
        Ok(())
    }

    async fn read_vote(
        &mut self,
    ) -> Result<Option<Vote<NodeRaftID>>, StorageError<NodeRaftID>> {
        Ok(self.vote.get())
    }

    async fn append_to_log<I>(
        &mut self,
        entries: I,
    ) -> Result<(), StorageError<NodeRaftID>>
    where
        I: IntoIterator<Item = Entry<TypeConfig>>,
    {
        for entry in entries {
            self.log
                .insert(entry.log_id.index, entry)
                .map_err(|(id, _)| StorageError::IO {
                    source: StorageIOError::new(
                        ErrorSubject::<NodeRaftID>::Store,
                        ErrorVerb::Write,
                        AnyError::error(format!(
                            "Already existant log with id {id}"
                        )),
                    ),
                });
        }
        Ok(())
    }

    async fn delete_conflict_logs_since(
        &mut self,
        log_id: LogId<NodeRaftID>,
    ) -> Result<(), StorageError<NodeRaftID>> {
        tracing::debug!("delete_log: [{:?}, +oo)", log_id);

        // TODO: Need to use the async version
        self.log.remove_range(log_id.index..);

        Ok(())
    }

    async fn purge_logs_upto(
        &mut self,
        log_id: LogId<NodeRaftID>,
    ) -> Result<(), StorageError<NodeRaftID>> {
        tracing::debug!("delete_log: [{:?}, +oo)", log_id);

        {
            self.last_purged_log_id.update(|old| {
                assert!(old <= Some(log_id));
                Some(log_id)
            });
        }

        {
            self.log.remove_range(..=log_id.index);
        }

        Ok(())
    }

    async fn last_applied_state(
        &mut self,
    ) -> Result<
        (
            Option<LogId<NodeRaftID>>,
            StoredMembership<NodeRaftID, LANNode>,
        ),
        StorageError<NodeRaftID>,
    > {
        let state_machine = self.state_machine.borrow();
        Ok((
            state_machine.last_applied_log,
            state_machine.last_membership.clone(),
        ))
    }

    async fn apply_to_state_machine(
        &mut self,
        entries: &[Entry<TypeConfig>],
    ) -> Result<Vec<ResponseLAN>, StorageError<NodeRaftID>> {
        let mut res = Vec::with_capacity(entries.len());

        for entry in entries {
            tracing::info!(%entry.log_id, "replicate to sm");

            self.state_machine.borrow_mut().last_applied_log =
                Some(entry.log_id);

            match entry.payload {
                EntryPayload::Blank => res.push(ResponseLAN::Nothing),
                EntryPayload::Normal(ref req) => match req {
                    RequestLAN::Nothing => res.push(ResponseLAN::Nothing),
                },
                EntryPayload::Membership(ref mem) => {
                    self.state_machine.borrow_mut().last_membership =
                        StoredMembership::new(Some(entry.log_id), mem.clone());
                    res.push(ResponseLAN::Nothing)
                }
            };
        }
        Ok(res)
    }

    async fn begin_receiving_snapshot(
        &mut self,
    ) -> Result<
        Box<<TypeConfig as RaftTypeConfig>::SnapshotData>,
        StorageError<NodeRaftID>,
    > {
        Ok(Box::new(Cursor::new(Vec::new())))
    }

    async fn install_snapshot(
        &mut self,
        meta: &SnapshotMeta<NodeRaftID, LANNode>,
        snapshot: Box<<TypeConfig as RaftTypeConfig>::SnapshotData>,
    ) -> Result<(), StorageError<NodeRaftID>> {
        /*
                tracing::info!(
                    { snapshot_size = snapshot.get_ref().len() },
                    "decoding snapshot for installation"
                );

                let new_snapshot = StoredSnapshot {
                    meta: meta.clone(),
                    data: snapshot.into_inner(),
                };

                // Update the state machine.
                {
                    let updated_state_machine: StateMachine =
                        serde_json::from_slice(&new_snapshot.data).map_err(|e| {
                            StorageIOError::read_snapshot(
                                Some(new_snapshot.meta.signature()),
                                &e,
                            )
                        })?;
                    let mut state_machine = self.state_machine.write().await;
                    *state_machine = updated_state_machine;
                }

                // Update current snapshot.
                let mut current_snapshot = self.current_snapshot.write().await;
                *current_snapshot = Some(new_snapshot);
                Ok(())
        */
        todo!()
    }

    async fn get_current_snapshot(
        &mut self,
    ) -> Result<Option<Snapshot<TypeConfig>>, StorageError<NodeRaftID>> {
        /*
                match &*self.current_snapshot.read().await {
                    Some(snapshot) => {
                        let data = snapshot.data.clone();
                        Ok(Some(Snapshot {
                            meta: snapshot.meta.clone(),
                            snapshot: Box::new(Cursor::new(data)),
                        }))
                    }
                    None => Ok(None),
                }
        */
        todo!()
    }

    async fn get_log_reader(&mut self) -> Self::LogReader {
        self.clone()
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        self.clone()
    }
}
