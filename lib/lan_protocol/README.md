# roster-lan-protocol

The LAN Protocol used to communicate in a
[roster](https://github.com/miaxos/roster) local cluster.

Each `roster` instance is able to handle some partitions, for each of those
partitions you can have multiple replicates depending on the load in the LAN
cluster.

The LAN cluster allow `roster` instances to:
  - Share which partitions each one is handling at one time
  - For each partitions there are one `leader` and one or many `followers`
  - For each partitions, depending on the nature of the command, we might prefer
  load balancing the command to a `follower` or to a `leader`

  - In case of [Dynamic Partitionning](#), the whole cluster is able to pilot
  partitionning and replicates accross the whole LAN cluster based on the size
  of each partition and the load of each one.

The LAN Cluster shouldn't have a lot of messages passing, as every message is
shared accross the whole cluster, we have a different process to ensure the
replication between `leader` and `follower` on each partition.

The LAN Cluster master decide itself when switching a `leader` to a follower and
a `follower` into a `leader`.

## Requests

(To be done)

## Response

(To be done)

## Dynamic partitionning

This is not done yet, the idea need to be documented, but it would basically
allow a `roster` cluster to self handle partitionning depending on some metrics.
