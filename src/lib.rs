pub(crate) mod client;
mod connection;
pub(crate) mod packet;
mod packetqueue;
pub mod packets;
pub(crate) mod ping;
pub(crate) mod server;
pub use crate::client::*;
pub use crate::ping::*;
pub use crate::server::*;
pub(crate) mod macros;
pub(crate) mod rak;
pub mod reader;
mod receivedqueue;
pub mod writer;
pub use crate::rak::*;

pub(crate) fn time() -> u128 {
    std::convert::TryInto::try_into(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    )
    .unwrap_or(0)
}
