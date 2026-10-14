//! Lab topology: devices, links and simple addressing (no simulation yet).

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceKind {
    Router,
    Switch,
    Host,
}

impl DeviceKind {
    pub const ALL: [DeviceKind; 3] = [DeviceKind::Router, DeviceKind::Switch, DeviceKind::Host];

    pub fn label(self) -> &'static str {
        match self {
            DeviceKind::Router => "Router",
            DeviceKind::Switch => "Switch",
            DeviceKind::Host => "Host",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: NodeId,
    pub kind: DeviceKind,
    pub name: String,
    /// Canvas position in lab units (not world CAD coordinates).
    pub x: f32,
    pub y: f32,
    pub ipv4: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LinkId(pub u64);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub id: LinkId,
    pub a: NodeId,
    pub b: NodeId,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topology {
    pub title: String,
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
    #[serde(skip)]
    next_node: u64,
    #[serde(skip)]
    next_link: u64,
}

impl Topology {
    pub fn new(title: impl Into<String>) -> Self {
        Topology { title: title.into(), ..Topology::default() }
    }

    pub fn add_node(&mut self, kind: DeviceKind, name: impl Into<String>, x: f32, y: f32) -> NodeId {
        let id = NodeId(self.next_node);
        self.next_node = self.next_node.saturating_add(1);
        self.nodes.push(Node { id, kind, name: name.into(), x, y, ipv4: None });
        id
    }

    pub fn add_link(&mut self, a: NodeId, b: NodeId) -> Result<LinkId, &'static str> {
        if a == b {
            return Err("a link needs two different nodes");
        }
        if !self.nodes.iter().any(|n| n.id == a) || !self.nodes.iter().any(|n| n.id == b) {
            return Err("unknown node");
        }
        let id = LinkId(self.next_link);
        self.next_link = self.next_link.saturating_add(1);
        self.links.push(Link { id, a, b });
        Ok(id)
    }

    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_requires_two_nodes() {
        let mut t = Topology::new("lab");
        let a = t.add_node(DeviceKind::Host, "pc1", 0.0, 0.0);
        assert!(t.add_link(a, a).is_err());
        let b = t.add_node(DeviceKind::Switch, "sw1", 10.0, 0.0);
        assert!(t.add_link(a, b).is_ok());
    }
}
