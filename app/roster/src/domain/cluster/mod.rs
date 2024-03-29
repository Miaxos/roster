mod lan_cluster;
pub use lan_cluster::LANCluster;

/// The global [Cluster] which will allow `roster` to distribute queries to
/// other servers if needed and ensure the coherence of the whole cluster by
/// replicating to replicas, load-balancing if needed and ensuring the cluster
/// is able to work.
///
/// **Should be cheap to clone**
#[derive(Debug, Clone)]
pub struct Cluster {}
