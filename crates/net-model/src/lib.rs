//! Lab topology: devices, links and IPv4 addressing.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![forbid(unsafe_code)]

mod ipv4;
mod mac;

use serde::{Deserialize, Serialize};

pub use ipv4::Ipv4Addr;
pub use mac::{MacAddr, node_mac};

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

    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    pub fn set_ipv4(&mut self, id: NodeId, addr: impl Into<String>) -> Result<(), &'static str> {
        let text = addr.into();
        if Ipv4Addr::parse(&text).is_none() {
            return Err("invalid IPv4 address");
        }
        let node = self.node_mut(id).ok_or("unknown node")?;
        node.ipv4 = Some(text);
        Ok(())
    }

    pub fn host_by_ipv4(&self, addr: &Ipv4Addr) -> Option<&Node> {
        self.nodes.iter().find(|n| {
            n.kind == DeviceKind::Host
                && n.ipv4.as_ref().and_then(|s| Ipv4Addr::parse(s)).as_ref() == Some(&*addr)
        })
    }

    /// Neighbours across links (undirected).
    pub fn neighbours(&self, id: NodeId) -> Vec<NodeId> {
        let mut out = Vec::new();
        for link in &self.links {
            if link.a == id {
                out.push(link.b);
            } else if link.b == id {
                out.push(link.a);
            }
        }
        out
    }

    pub fn find_link(&self, a: NodeId, b: NodeId) -> Option<LinkId> {
        self.links.iter().find(|l| (l.a == a && l.b == b) || (l.a == b && l.b == a)).map(|l| l.id)
    }

    /// Each cable attached to `device` (one port per link).
    pub fn ports(&self, device: NodeId) -> Vec<LinkEndpoint> {
        let mut out = Vec::new();
        for link in &self.links {
            if link.a == device {
                out.push(LinkEndpoint { link_id: link.id, peer: link.b });
            } else if link.b == device {
                out.push(LinkEndpoint { link_id: link.id, peer: link.a });
            }
        }
        out
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkEndpoint {
    pub link_id: LinkId,
    pub peer: NodeId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipv4_parse_rejects_garbage() {
        assert!(Ipv4Addr::parse("10.0.0.1").is_some());
        assert!(Ipv4Addr::parse("999.0.0.1").is_none());
        assert!(Ipv4Addr::parse("").is_none());
    }

    #[test]
    fn link_requires_two_nodes() {
        let mut t = Topology::new("lab");
        let a = t.add_node(DeviceKind::Host, "pc1", 0.0, 0.0);
        assert!(t.add_link(a, a).is_err());
        let b = t.add_node(DeviceKind::Switch, "sw1", 10.0, 0.0);
        assert!(t.add_link(a, b).is_ok());
    }

    #[test]
    fn ports_list_links_per_device() {
        let mut t = Topology::new("lab");
        let sw = t.add_node(DeviceKind::Switch, "sw", 0.0, 0.0);
        let pc = t.add_node(DeviceKind::Host, "pc", 0.0, 0.0);
        let link = t.add_link(pc, sw).expect("link");
        assert_eq!(t.ports(pc).len(), 1);
        assert_eq!(t.ports(pc)[0].link_id, link);
        assert_eq!(t.ports(pc)[0].peer, sw);
    }
}
