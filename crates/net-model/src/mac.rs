//! Ethernet MAC addresses for L2 simulation.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::NodeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MacAddr(pub [u8; 6]);

impl MacAddr {
    pub const BROADCAST: MacAddr = MacAddr([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);

    pub fn is_broadcast(self) -> bool {
        self == MacAddr::BROADCAST
    }
}

/// Stable MAC derived from a lab node id (not a real vendor OUI).
pub fn node_mac(id: NodeId) -> MacAddr {
    let n = id.0;
    MacAddr([0xaa, 0xbb, 0xcc, 0xdd, (n >> 8) as u8, (n & 0xff) as u8])
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5]
        )
    }
}
