use sharded_thread::shard::Shard;

use super::handle::ConnectionMsg;
use super::ServerConfig;
use crate::domain::dialer::Dialer;

/// This structure is used only in a single thread.
pub struct ServerMonoThreadedHandle {
    config: ServerConfig,
    /// Communication layer which will know if we need to send the message
    /// through another thread or through another TCP connection.
    dial: Dialer,
    /// The cpu where the thread should be binded here.
    cpu: usize,
}
