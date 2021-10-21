#[cfg(unix)]
mod cmsg;
mod proto;
#[cfg(feature = "with-async-io")]
mod socket;
#[cfg(unix)]
pub mod unix;

pub use proto::{EcnCodepoint, RecvMeta, SocketType, Transmit, UdpCapabilities};
#[cfg(feature = "with-async-io")]
pub use socket::UdpSocket;

/// Number of UDP packets to send/receive at a time when using sendmmsg/recvmmsg.
pub const BATCH_SIZE: usize = {
    if cfg!(target_os = "linux") {
        // Chosen somewhat arbitrarily; might benefit from additional tuning.
        32
    } else {
        1
    }
};
