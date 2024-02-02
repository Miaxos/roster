# Roster Cluster Specification

First of all, Roster cluster is not a Redis Cluster. Even if you are able to
interact with a Roster cluster in the same way as with a Redis Cluster, we don't
implement the protocol allowing roster to be integrated inside a Redis Cluster.

## Main goals

- High performances
- Availability
- Handling of strongly replicated data and weakly replicated data.
- ACL must be highly consistent

### ACL Replication

A Roster Cluster got his ACL globally replicated in every node through a Raft
protocol.
